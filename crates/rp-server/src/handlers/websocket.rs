use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::StreamExt;
use std::sync::Arc;

use crate::state::AppState;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(_state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    // TODO: Implement WebSocket protocol
    let (_sender, mut _receiver) = socket.split();
    
    // For now, just accept the connection and close it
    tracing::info!("WebSocket connection established");
}