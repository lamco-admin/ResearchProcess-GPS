use sqlx::{postgres::PgListener, PgPool};
use tokio::sync::broadcast;
use tracing::{debug, error, info};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventNotification {
    pub event_id: Uuid,
    pub aggregate_id: Uuid,
    pub aggregate_type: String,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// Start listening for PostgreSQL notifications
pub async fn start_notification_listener(
    pool: PgPool,
    event_tx: broadcast::Sender<EventNotification>,
) -> Result<(), sqlx::Error> {
    let mut listener = PgListener::connect_with(&pool).await?;
    
    // Listen to the 'events' channel
    listener.listen("events").await?;
    
    info!("Started PostgreSQL LISTEN on 'events' channel");
    
    // Spawn a task to handle notifications
    tokio::spawn(async move {
        loop {
            match listener.recv().await {
                Ok(notification) => {
                    // Parse the notification payload
                    match serde_json::from_str::<EventNotification>(&notification.payload()) {
                        Ok(event) => {
                            info!(
                                "Received event notification: {} for aggregate {}",
                                event.event_type, event.aggregate_id
                            );
                            
                            // Broadcast to all WebSocket connections
                            match event_tx.send(event) {
                                Ok(receiver_count) => {
                                    if receiver_count > 0 {
                                        info!("Broadcast event to {} receivers", receiver_count);
                                    }
                                }
                                Err(e) => {
                                    // This is expected if no WebSocket clients are connected
                                    debug!("No receivers for event broadcast: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse notification payload: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Error receiving notification: {}", e);
                    // Try to reconnect after a delay
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }
    });
    
    Ok(())
}