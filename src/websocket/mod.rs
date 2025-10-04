pub mod helius_stream;
pub mod server;

pub use helius_stream::HeliusWebSocket;
pub use server::{WebSocketState, broadcast_to_clients};

use serde::{Deserialize, Serialize};

// Messages WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionUpdate {
    pub timestamp: String,
    pub tps: f64,
    pub recent_transactions: Vec<RecentTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentTransaction {
    pub signature: String,
    pub contract: String,
    pub amount_sol: f64,
    pub from: String,
    pub to: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkActivity {
    pub time: String,
    pub tps: f64,
    pub transactions: u64,
}