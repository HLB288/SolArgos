use super::models::*;
use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub async fn get_solana_metrics(api_key: String) -> Result<SolanaMetrics, Box<dyn Error>> {
    let client = Client::new();
    
    // Données simulées pour test rapide
    Ok(SolanaMetrics {
        total_transactions: 264892147,
        current_slot: 295841623,
        estimated_tps: 3042.0,
        biggest_transaction_sol: 15432.85,
        biggest_transaction_slot: 295841523,
        biggest_transaction_time: "03/10/2025 14:32:15".to_string(),
        performance_samples_count: 720,
        performance_period_hours: 12.0,
        non_vote_transactions: 45120000,
        network_status: NetworkStatus::Excellent,
        last_update: chrono::Utc::now().format("%d/%m/%Y %H:%M:%S").to_string(),
        analysis_duration_ms: 100,
    })
}