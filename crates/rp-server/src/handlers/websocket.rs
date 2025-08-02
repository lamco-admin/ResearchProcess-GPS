use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use rp_protocol::{
    ClientMessage, ClientMessagePayload, ServerMessage, ServerMessagePayload,
    ErrorMessage, ErrorCode, SubscriptionRequest, SubscriptionType,
    EventNotification as ProtoEventNotification, EventType, EventMetadata as ProtoEventMetadata,
};
use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

use crate::state::AppState;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Connection state for a WebSocket client
struct ConnectionState {
    authenticated: bool,
    subscriptions: HashMap<String, SubscriptionRequest>,
    actor_id: Option<Uuid>,
}

async fn handle_socket(socket: WebSocket, app_state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let connection_id = Uuid::new_v4();
    
    tracing::info!("WebSocket connection established: {}", connection_id);
    
    // Create channels for this connection
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();
    let connection_state = Arc::new(RwLock::new(ConnectionState {
        authenticated: false,
        subscriptions: HashMap::new(),
        actor_id: None,
    }));
    
    // Subscribe to event notifications
    let mut event_rx = app_state.event_tx.subscribe();
    
    // Spawn task to forward messages from channel to WebSocket
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });
    
    // Spawn task to forward events to this connection
    let mut event_task = {
        let tx = tx.clone();
        let state = connection_state.clone();
        
        tokio::spawn(async move {
            while let Ok(event) = event_rx.recv().await {
                // Check if this connection is interested in this event
                let should_send = {
                    let conn_state = state.read().await;
                    if !conn_state.authenticated {
                        false
                    } else {
                        // Check each subscription
                        conn_state.subscriptions.values().any(|sub| {
                            match &sub.subscription_type {
                                SubscriptionType::Entity => {
                                    sub.params.entity_id == Some(event.aggregate_id)
                                }
                                SubscriptionType::EntityType => {
                                    if let Some(ref expected_type) = sub.params.entity_type {
                                        expected_type == &event.aggregate_type
                                    } else {
                                        false
                                    }
                                }
                                SubscriptionType::Workspace => {
                                    // TODO: Check workspace_id when available
                                    false
                                }
                                SubscriptionType::Query => {
                                    // TODO: Implement query matching
                                    false
                                }
                            }
                        })
                    }
                };
                
                if should_send {
                    // Validate required fields - skip event if missing
                    if let (Some(version), Some(actor_id)) = (event.version, event.actor_id) {
                        // Convert to protocol event notification
                        let proto_event = ProtoEventNotification {
                            event_id: event.event_id,
                            event_type: match event.event_type.as_str() {
                                "created" => EventType::Created,
                                "updated" => EventType::Updated,
                                "deleted" => EventType::Deleted,
                                _ => EventType::Custom(event.event_type.clone()),
                            },
                            entity_type: event.aggregate_type.clone(),
                            entity_id: event.aggregate_id,
                            version,
                            data: event.event_data.clone(),
                            metadata: ProtoEventMetadata {
                                occurred_at: event.occurred_at,
                                actor_id,
                                workspace_id: None,
                                correlation_id: None,
                                tags: None,
                            },
                        };

                        let msg = ServerMessage {
                            id: None,
                            payload: ServerMessagePayload::Event(proto_event),
                        };
                        
                        let _ = tx.send(msg);
                    } else {
                        // Log warning about missing required fields
                        tracing::warn!(
                            "Skipping event {} - missing required fields (version: {:?}, actor_id: {:?})",
                            event.event_id,
                            event.version,
                            event.actor_id
                        );
                    }
                }
            }
        })
    };
    
    // Main message handling loop
    let mut recv_task = {
        let tx = tx.clone();
        let state = connection_state.clone();
        let app_state = app_state.clone();
        
        tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                if let Message::Text(text) = msg {
                    handle_client_message(
                        &text,
                        &tx,
                        &state,
                        &app_state,
                        connection_id,
                    ).await;
                } else if let Message::Close(_) = msg {
                    break;
                }
            }
        })
    };
    
    // Wait for any task to complete
    tokio::select! {
        _ = &mut send_task => {
            recv_task.abort();
            event_task.abort();
        }
        _ = &mut recv_task => {
            send_task.abort();
            event_task.abort();
        }
        _ = &mut event_task => {
            send_task.abort();
            recv_task.abort();
        }
    }
    
    tracing::info!("WebSocket connection closed: {}", connection_id);
}

async fn handle_client_message(
    text: &str,
    tx: &mpsc::UnboundedSender<ServerMessage>,
    connection_state: &Arc<RwLock<ConnectionState>>,
    app_state: &Arc<AppState>,
    connection_id: Uuid,
) {
    // Parse the client message
    let msg: ClientMessage = match serde_json::from_str(text) {
        Ok(msg) => msg,
        Err(e) => {
            let error = ServerMessage {
                id: None,
                payload: ServerMessagePayload::Error(ErrorMessage {
                    code: ErrorCode::ProtocolError,
                    message: format!("Invalid JSON: {}", e),
                    details: None,
                    request_id: None,
                }),
            };
            let _ = tx.send(error);
            return;
        }
    };
    
    // Check authentication for non-auth messages
    let is_authenticated = connection_state.read().await.authenticated;
    if !is_authenticated && !matches!(&msg.payload, ClientMessagePayload::Auth { .. }) {
        let error = ServerMessage {
            id: Some(msg.id.clone()),
            payload: ServerMessagePayload::Error(ErrorMessage {
                code: ErrorCode::AuthenticationError,
                message: "Not authenticated".to_string(),
                details: None,
                request_id: Some(msg.id),
            }),
        };
        let _ = tx.send(error);
        return;
    }
    
    // Handle message based on type
    match msg.payload {
        ClientMessagePayload::Auth { token } => {
            handle_auth(token, &msg.id, tx, connection_state).await;
        }
        ClientMessagePayload::Subscribe(request) => {
            handle_subscribe(request, &msg.id, tx, connection_state, app_state).await;
        }
        ClientMessagePayload::Unsubscribe { subscription_id } => {
            handle_unsubscribe(subscription_id, &msg.id, tx, connection_state).await;
        }
        ClientMessagePayload::Pong { timestamp } => {
            tracing::debug!("Received pong from {}: {:?}", connection_id, timestamp);
        }
        ClientMessagePayload::Request { .. } => {
            // TODO: Implement request handling
            let error = ServerMessage {
                id: Some(msg.id.clone()),
                payload: ServerMessagePayload::Error(ErrorMessage {
                    code: ErrorCode::InternalError,
                    message: "Request handling not yet implemented".to_string(),
                    details: None,
                    request_id: Some(msg.id),
                }),
            };
            let _ = tx.send(error);
        }
    }
}

async fn handle_auth(
    token: String,
    request_id: &str,
    tx: &mpsc::UnboundedSender<ServerMessage>,
    connection_state: &Arc<RwLock<ConnectionState>>,
) {
    // TODO: Implement proper authentication
    // For now, accept any non-empty token
    let success = !token.is_empty();
    
    if success {
        let mut state = connection_state.write().await;
        state.authenticated = true;
        state.actor_id = Some(Uuid::nil()); // TODO: Get from token
    }
    
    let response = ServerMessage {
        id: Some(request_id.to_string()),
        payload: ServerMessagePayload::AuthResult {
            success,
            error: if success { None } else { Some("Invalid token".to_string()) },
        },
    };
    
    let _ = tx.send(response);
}

async fn handle_subscribe(
    request: SubscriptionRequest,
    request_id: &str,
    tx: &mpsc::UnboundedSender<ServerMessage>,
    connection_state: &Arc<RwLock<ConnectionState>>,
    _app_state: &Arc<AppState>,
) {
    let subscription_id = Uuid::new_v4().to_string();
    
    // Store subscription
    {
        let mut state = connection_state.write().await;
        state.subscriptions.insert(subscription_id.clone(), request.clone());
    }
    
    // TODO: Set up actual event streaming based on subscription type
    // For now, just confirm the subscription
    let response = ServerMessage {
        id: Some(request_id.to_string()),
        payload: ServerMessagePayload::SubscriptionCreated {
            subscription_id,
            existing_data: None, // TODO: Load existing data based on subscription
        },
    };
    
    let _ = tx.send(response);
}

async fn handle_unsubscribe(
    subscription_id: String,
    request_id: &str,
    tx: &mpsc::UnboundedSender<ServerMessage>,
    connection_state: &Arc<RwLock<ConnectionState>>,
) {
    let removed = {
        let mut state = connection_state.write().await;
        state.subscriptions.remove(&subscription_id).is_some()
    };
    
    if removed {
        let response = ServerMessage {
            id: Some(request_id.to_string()),
            payload: ServerMessagePayload::SubscriptionRemoved { subscription_id },
        };
        let _ = tx.send(response);
    } else {
        let error = ServerMessage {
            id: Some(request_id.to_string()),
            payload: ServerMessagePayload::Error(ErrorMessage {
                code: ErrorCode::NotFound,
                message: "Subscription not found".to_string(),
                details: None,
                request_id: Some(request_id.to_string()),
            }),
        };
        let _ = tx.send(error);
    }
}