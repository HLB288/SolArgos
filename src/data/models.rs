use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaMetrics {
    pub total_transactions: u64,
    pub current_slot: u64,
    pub estimated_tps: f64,
    pub biggest_transaction_sol: f64,
    pub biggest_transaction_slot: u64,
    pub biggest_transaction_time: String,
    pub performance_samples_count: usize,
    pub performance_period_hours: f64,
    pub non_vote_transactions: u64,
    pub network_status: NetworkStatus,
    pub last_update: String,
    pub analysis_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkStatus {
    Excellent,
    Good,
    Moderate,
    Degraded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSample {
    pub slot: u64,
    #[serde(rename = "numTransactions")]
    pub num_transactions: u64,
    #[serde(rename = "numNonVoteTransactions")]
    pub num_non_vote_transactions: u64,
    #[serde(rename = "samplePeriodSecs")]
    pub sample_period_secs: u64,
}

#[derive(Debug, Clone)]
pub struct SolanaAnalysis {
    pub performance_total_tx: u64,
    pub performance_non_vote_tx: u64,
    pub performance_tps: f64,
    pub performance_period_hours: f64,
    pub global_total_tx: u64,
    pub contracts_sample: u64,
    pub final_estimate_24h: u64,
    pub final_tps_avg: f64,
    pub confidence_score: f32,
    pub analysis_time_ms: u128,
    pub anomalies: Vec<String>,
    pub trend: String,
}

pub fn format_number(num: u64) -> String {
    num.to_string()
        .chars()
        .rev()
        .collect::<String>()
        .as_bytes()
        .chunks(3)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .rev()
        .collect::<String>()
}