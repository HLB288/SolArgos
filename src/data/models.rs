// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

// // ===============================
// // STRUCTURES RPC HELIUS
// // ===============================

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct PerformanceSample {
//     pub slot: u64,
//     #[serde(rename = "numTransactions")]
//     pub num_transactions: u64,
//     #[serde(rename = "numNonVoteTransactions")]
//     pub num_non_vote_transactions: u64,
//     #[serde(rename = "samplePeriodSecs")]
//     pub sample_period_secs: u64,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct PerformanceSamplesResponse {
//     pub jsonrpc: String,
//     pub id: String,
//     pub result: Vec<PerformanceSample>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct TransactionCountResponse {
//     pub jsonrpc: String,
//     pub id: String,
//     pub result: u64,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SignatureInfo {
//     pub signature: String,
//     pub slot: Option<u64>,
//     #[serde(rename = "blockTime")]
//     pub block_time: Option<i64>,
//     pub err: Option<serde_json::Value>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SignaturesResponse {
//     pub jsonrpc: String,
//     pub id: String,
//     pub result: Vec<SignatureInfo>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SlotResponse {
//     pub jsonrpc: String,
//     pub id: String,
//     pub result: u64,
// }

// // ===============================
// // STRUCTURES DE DONNÃ‰ES MÃ‰TIER
// // ===============================

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SolanaMetrics {
//     pub total_transactions: u64,
//     pub current_slot: u64,
//     pub estimated_tps: f64,
//     pub biggest_transaction_sol: f64,
//     pub biggest_transaction_slot: u64,
//     pub biggest_transaction_time: String,
//     pub performance_samples_count: usize,
//     pub performance_period_hours: f64,
//     pub non_vote_transactions: u64,
//     pub network_status: NetworkStatus,
//     pub last_update: String,
//     pub analysis_duration_ms: u64,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "lowercase")]
// pub enum NetworkStatus {
//     Excellent,  // > 3500 TPS
//     Good,       // > 2500 TPS
//     Moderate,   // > 1500 TPS
//     Degraded,   // <= 1500 TPS
// }

// impl NetworkStatus {
//     pub fn from_tps(tps: f64) -> Self {
//         if tps > 3500.0 {
//             NetworkStatus::Excellent
//         } else if tps > 2500.0 {
//             NetworkStatus::Good
//         } else if tps > 1500.0 {
//             NetworkStatus::Moderate
//         } else {
//             NetworkStatus::Degraded
//         }
//     }

//     pub fn to_emoji(&self) -> &'static str {
//         match self {
//             NetworkStatus::Excellent => "🟢",
//             NetworkStatus::Good => "🟡",
//             NetworkStatus::Moderate => "🟠",
//             NetworkStatus::Degraded => "🔴",
//         }
//     }

//     pub fn to_text(&self) -> &'static str {
//         match self {
//             NetworkStatus::Excellent => "Excellent",
//             NetworkStatus::Good => "Bon",
//             NetworkStatus::Moderate => "Modéré",
//             NetworkStatus::Degraded => "Dégradé",
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct SolanaAnalysis {
//     pub performance_total_tx: u64,
//     pub performance_non_vote_tx: u64,
//     pub performance_tps: f64,
//     pub performance_period_hours: f64,
//     pub global_total_tx: u64,
//     pub contracts_sample: u64,
//     pub final_estimate_24h: u64,
//     pub final_tps_avg: f64,
//     pub confidence_score: f32,
//     pub analysis_time_ms: u128,
//     pub anomalies: Vec<String>,
//     pub trend: String,
// }

// // ===============================
// // CONSTANTES
// // ===============================

// pub const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;

// // Contrats populaires pour l'échantillonnage
// pub const MAJOR_CONTRACTS: [&str; 3] = [
//     "So11111111111111111111111111111111111111112",     // Wrapped SOL
//     "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",  // USDC
//     "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB",   // Jupiter
// ];

// // ===============================
// // UTILITAIRES
// // ===============================

// pub fn format_number(num: u64) -> String {
//     num.to_string()
//         .chars()
//         .rev()
//         .collect::<String>()
//         .as_bytes()
//         .chunks(3)
//         .map(|chunk| std::str::from_utf8(chunk).unwrap())
//         .collect::<Vec<_>>()
//         .join(" ")
//         .chars()
//         .rev()
//         .collect::<String>()
// }

// pub fn format_timestamp(timestamp: i64) -> String {
//     use chrono::{DateTime, Utc};
    
//     match DateTime::<Utc>::from_timestamp(timestamp, 0) {
//         Some(datetime) => datetime.format("%d/%m/%Y %H:%M:%S").to_string(),
//         None => "N/A".to_string(),
//     }
// }

// impl Default for SolanaMetrics {
//     fn default() -> Self {
//         Self {
//             total_transactions: 0,
//             current_slot: 0,
//             estimated_tps: 0.0,
//             biggest_transaction_sol: 0.0,
//             biggest_transaction_slot: 0,
//             biggest_transaction_time: "N/A".to_string(),
//             performance_samples_count: 0,
//             performance_period_hours: 0.0,
//             non_vote_transactions: 0,
//             network_status: NetworkStatus::Degraded,
//             last_update: "N/A".to_string(),
//             analysis_duration_ms: 0,
//         }
//     }
// }

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