use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade, State},
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};

use super::{TransactionUpdate, RecentTransaction};

#[derive(Clone)]
pub struct WebSocketState {
    pub recent_transactions: Vec<RecentTransaction>,
    pub tx: broadcast::Sender<String>,
}

impl WebSocketState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            recent_transactions: Vec::new(),
            tx,
        }
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<Mutex<WebSocketState>>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<Mutex<WebSocketState>>) {
    let (mut sender, mut receiver) = socket.split();
    
    // S'abonner au broadcast
    let state_guard = state.lock().await;
    let mut rx = state_guard.tx.subscribe();
    drop(state_guard);

    println!("✅ Nouveau client WebSocket connecté");

    // Tâche pour envoyer les messages
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender
                .send(axum::extract::ws::Message::Text(msg))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    // Tâche pour recevoir les messages (ping/pong)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                axum::extract::ws::Message::Close(_) => break,
                _ => {}
            }
        }
    });

    // Attendre qu'une tâche se termine
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    println!("❌ Client WebSocket déconnecté");
}

pub async fn broadcast_to_clients(state: &WebSocketState, update: &TransactionUpdate) {
    if let Ok(json) = serde_json::to_string(update) {
        let _ = state.tx.send(json);
    }
}