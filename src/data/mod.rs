// Module pour la gestion des données
// On intégrera ton code solana_client.rs ici

// Pour plus tard :
// pub mod solana_client;
// pub mod database;
// pub mod cache;

// Structures de données pour l'instant
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaMetrics {
    pub total_transactions: u64,
    pub current_slot: u64,
    pub estimated_tps: f64,
    pub token_counts: HashMap<String, u64>,
    pub biggest_transaction_sol: f64,
    pub biggest_transaction_slot: u64,
    pub biggest_transaction_time: String,
    pub last_update: String,
    pub analysis_duration: u64,
}

impl Default for SolanaMetrics {
    fn default() -> Self {
        Self {
            total_transactions: 0,
            current_slot: 0,
            estimated_tps: 0.0,
            token_counts: HashMap::new(),
            biggest_transaction_sol: 0.0,
            biggest_transaction_slot: 0,
            biggest_transaction_time: "N/A".to_string(),
            last_update: "N/A".to_string(),
            analysis_duration: 0,
        }
    }
}