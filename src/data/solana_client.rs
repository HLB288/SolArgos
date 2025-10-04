use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub struct HeliusClient {
    client: Client,
    api_key: String,
}

impl HeliusClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn get_performance_samples(&self, limit: u64) -> Result<Vec<super::models::PerformanceSample>, Box<dyn Error>> {
        let response = self.client
            .post("https://mainnet.helius-rpc.com/")
            .header("Content-Type", "application/json")
            .query(&[("api-key", &self.api_key)])
            .json(&json!({
                "jsonrpc": "2.0",
                "id": "1",
                "method": "getRecentPerformanceSamples",
                "params": [limit]
            }))
            .send()
            .await?
            .json::<super::models::PerformanceSamplesResponse>()
            .await?;

        Ok(response.result)
    }
}
pub async fn get_solana_metrics(api_key: String) -> Result<super::models::SolanaMetrics, Box<dyn Error>> {
    let _client = Client::new();
    
    // Version simplifiée pour test
    Ok(super::models::SolanaMetrics {
        total_transactions: 264892147,
        current_slot: 295841623,
        estimated_tps: 3042.0,
        biggest_transaction_sol: 15432.85,
        biggest_transaction_slot: 295841523,
        biggest_transaction_time: "03/10/2025 14:32:15".to_string(),
        performance_samples_count: 720,
        performance_period_hours: 12.0,
        non_vote_transactions: 45120000,
        network_status: super::models::NetworkStatus::Excellent,
        last_update: chrono::Utc::now().format("%d/%m/%Y %H:%M:%S").to_string(),
        analysis_duration_ms: 100,
    })
}