// use super::models::*;
// use reqwest::Client;
// use serde_json::json;
// use std::error::Error;

// // ===============================
// // CLIENT HELIUS RPC
// // ===============================

// pub struct HeliusClient {
//     client: Client,
//     api_key: String,
//     base_url: String,
// }

// impl HeliusClient {
//     /// Créer un nouveau client Helius
//     /// 
//     /// # Arguments
//     /// * `api_key` - Clé API Helius
//     pub fn new(api_key: String) -> Self {
//         Self {
//             client: Client::new(),
//             api_key,
//             base_url: "https://mainnet.helius-rpc.com".to_string(),
//         }
//     }

//     /// Récupérer le slot actuel (hauteur de bloc)
//     pub async fn get_current_slot(&self) -> Result<u64, Box<dyn Error>> {
//         let response = self.client
//             .post(&self.base_url)
//             .header("Content-Type", "application/json")
//             .query(&[("api-key", &self.api_key)])
//             .json(&json!({
//                 "jsonrpc": "2.0",
//                 "id": "1",
//                 "method": "getSlot",
//                 "params": [{"commitment": "finalized"}]
//             }))
//             .send()
//             .await?
//             .json::<SlotResponse>()
//             .await?;

//         Ok(response.result)
//     }

//     /// Récupérer le nombre total de transactions historiques
//     pub async fn get_transaction_count(&self) -> Result<u64, Box<dyn Error>> {
//         let response = self.client
//             .post(&self.base_url)
//             .header("Content-Type", "application/json")
//             .query(&[("api-key", &self.api_key)])
//             .json(&json!({
//                 "jsonrpc": "2.0",
//                 "id": "1",
//                 "method": "getTransactionCount",
//                 "params": [{"commitment": "finalized"}]
//             }))
//             .send()
//             .await?
//             .json::<TransactionCountResponse>()
//             .await?;

//         Ok(response.result)
//     }

//     /// Récupérer les échantillons de performance récents
//     /// 
//     /// # Arguments
//     /// * `limit` - Nombre d'échantillons à récupérer (max 720)
//     pub async fn get_performance_samples(&self, limit: u64) -> Result<Vec<PerformanceSample>, Box<dyn Error>> {
//         let response = self.client
//             .post(&self.base_url)
//             .header("Content-Type", "application/json")
//             .query(&[("api-key", &self.api_key)])
//             .json(&json!({
//                 "jsonrpc": "2.0",
//                 "id": "1",
//                 "method": "getRecentPerformanceSamples",
//                 "params": [limit]
//             }))
//             .send()
//             .await?
//             .json::<PerformanceSamplesResponse>()
//             .await?;

//         Ok(response.result)
//     }

//     /// Récupérer les signatures pour une adresse (contrat ou wallet)
//     /// 
//     /// # Arguments
//     /// * `address` - Adresse Solana
//     /// * `limit` - Nombre de signatures à récupérer
//     pub async fn get_signatures_for_address(
//         &self, 
//         address: &str, 
//         limit: u64
//     ) -> Result<Vec<SignatureInfo>, Box<dyn Error>> {
//         let response = self.client
//             .post(&self.base_url)
//             .header("Content-Type", "application/json")
//             .query(&[("api-key", &self.api_key)])
//             .json(&json!({
//                 "jsonrpc": "2.0",
//                 "id": "1",
//                 "method": "getSignaturesForAddress",
//                 "params": [
//                     address,
//                     {"limit": limit}
//                 ]
//             }))
//             .send()
//             .await?
//             .json::<SignaturesResponse>()
//             .await?;

//         Ok(response.result)
//     }
// }

// // ===============================
// // FONCTIONS D'ANALYSE PRINCIPALES
// // ===============================

// /// Analyser les échantillons de performance pour calculer TPS et volumes
// pub async fn analyze_performance_samples(
//     client: &HeliusClient
// ) -> Result<(u64, u64, f64, f64, Vec<PerformanceSample>), Box<dyn Error>> {
//     println!("📊 Analyse Performance Samples...");
    
//     let samples = client.get_performance_samples(720).await?;
    
//     if samples.is_empty() {
//         return Err("Aucun échantillon reçu".into());
//     }
    
//     // Calculer les totaux
//     let total_period_secs: u64 = samples.iter().map(|s| s.sample_period_secs).sum();
//     let total_hours = total_period_secs as f64 / 3600.0;
    
//     let total_transactions: u64 = samples.iter().map(|s| s.num_transactions).sum();
//     let total_non_vote: u64 = samples.iter().map(|s| s.num_non_vote_transactions).sum();
    
//     // Calculer TPS réel moyen
//     let tps_real = if total_period_secs > 0 {
//         total_transactions as f64 / total_period_secs as f64
//     } else {
//         0.0
//     };
    
//     println!("   ✅ {} échantillons sur {:.1}h", samples.len(), total_hours);
//     println!("   ⚡ TPS réel moyen: {:.0}", tps_real);
//     println!("   📊 Total observé: {} tx", format_number(total_transactions));
    
//     Ok((total_transactions, total_non_vote, tps_real, total_hours, samples))
// }

// /// Échantillonnage rapide de contrats populaires pour validation
// pub async fn quick_contracts_sampling(
//     client: &HeliusClient
// ) -> Result<u64, Box<dyn Error>> {
//     println!("🔍 Échantillonnage contrats populaires...");
    
//     let mut total_recent = 0u64;
    
//     for contract in MAJOR_CONTRACTS.iter() {
//         match client.get_signatures_for_address(contract, 100).await {
//             Ok(signatures) => {
//                 total_recent += signatures.len() as u64;
//             }
//             Err(e) => {
//                 println!("   ⚠️ Erreur pour {}: {}", contract, e);
//             }
//         }
        
//         // Petit délai pour éviter le rate limiting
//         tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
//     }
    
//     println!("   📦 Échantillon collecté: {} signatures récentes", total_recent);
//     Ok(total_recent)
// }

// // ===============================
// // FONCTION PRINCIPALE D'ANALYSE COMPLÈTE
// // ===============================

// /// Récupérer toutes les métriques Solana via Helius
// pub async fn get_solana_metrics(api_key: String) -> Result<SolanaMetrics, Box<dyn Error>> {
//     let start_time = std::time::Instant::now();
//     let client = HeliusClient::new(api_key);

//     println!("🚀 Démarrage analyse Solana...");

//     // 1. Récupérer le total historique
//     let total_transactions = match client.get_transaction_count().await {
//         Ok(count) => {
//             println!("📈 Total historique: {}", format_number(count));
//             count
//         }
//         Err(e) => {
//             println!("⚠️ Erreur total transactions: {}", e);
//             0
//         }
//     };

//     // 2. Slot actuel
//     let current_slot = match client.get_current_slot().await {
//         Ok(slot) => {
//             println!("🎯 Slot actuel: {}", format_number(slot));
//             slot
//         }
//         Err(e) => {
//             println!("⚠️ Erreur slot actuel: {}", e);
//             0
//         }
//     };

//     // 3. Analyse de performance détaillée
//     let (perf_tx, perf_non_vote, performance_tps, perf_hours, samples) = 
//         match analyze_performance_samples(&client).await {
//             Ok(data) => data,
//             Err(e) => {
//                 println!("⚠️ Erreur performance samples: {}", e);
//                 (0, 0, 0.0, 0.0, Vec::new())
//             }
//         };

//     // 4. Échantillonnage contrats (optionnel)
//     let _contracts_sample = match quick_contracts_sampling(&client).await {
//         Ok(sample) => sample,
//         Err(e) => {
//             println!("⚠️ Erreur échantillonnage: {}", e);
//             0
//         }
//     };

//     // 5. Calculer le TPS estimé
//     let estimated_tps = if performance_tps > 0.0 {
//         performance_tps
//     } else {
//         2500.0 // Valeur par défaut conservatrice
//     };

//     // 6. Déterminer le statut du réseau
//     let network_status = NetworkStatus::from_tps(estimated_tps);

//     // 7. Plus grosse transaction (simulation pour l'instant)
//     // TODO: Implémenter la recherche réelle de la plus grosse transaction
//     let biggest_transaction_sol = 15432.85;
//     let biggest_transaction_slot = current_slot.saturating_sub(100);

//     let analysis_duration = start_time.elapsed().as_millis() as u64;
//     let now = chrono::Utc::now().timestamp();

//     println!("✅ Analyse terminée en {}ms", analysis_duration);
//     println!("🎯 TPS moyen: {:.0} - Statut: {} {}", 
//              estimated_tps, 
//              network_status.to_emoji(), 
//              network_status.to_text());

//     Ok(SolanaMetrics {
//         total_transactions,
//         current_slot,
//         estimated_tps,
//         biggest_transaction_sol,
//         biggest_transaction_slot,
//         biggest_transaction_time: format_timestamp(now),
//         performance_samples_count: samples.len(),
//         performance_period_hours: perf_hours,
//         non_vote_transactions: perf_non_vote,
//         network_status,
//         last_update: format_timestamp(now),
//         analysis_duration_ms: analysis_duration,
//     })
// }

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