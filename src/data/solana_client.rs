use super::{
    PerformanceSamplesResponse, TransactionCountResponse, SignaturesResponse,
    SolanaMetrics, HELIUS_API_KEY, LAMPORTS_PER_SOL, format_timestamp, format_number
};
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;

// ===============================
// CLIENT HELIUS RPC
// ===============================

pub struct HeliusClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl HeliusClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            api_key: HELIUS_API_KEY.to_string(),
            base_url: "https://mainnet.helius-rpc.com".to_string(),
        }
    }

    pub async fn get_current_slot(&self) -> Result<u64, Box<dyn Error>> {
        let response = self.client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .query(&[("api-key", &self.api_key)])
            .json(&json!({
                "jsonrpc": "2.0",
                "id": "1",
                "method": "getSlot",
                "params": [{"commitment": "finalized"}]
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(response["result"].as_u64().unwrap_or(0))
    }

    pub async fn get_transaction_count(&self) -> Result<u64, Box<dyn Error>> {
        let response = self.client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .query(&[("api-key", &self.api_key)])
            .json(&json!({
                "jsonrpc": "2.0",
                "id": "1",
                "method": "getTransactionCount",
                "params": [{"commitment": "finalized"}]
            }))
            .send()
            .await?
            .json::<TransactionCountResponse>()
            .await?;

        Ok(response.result)
    }

    pub async fn get_performance_samples(&self, limit: u64) -> Result<PerformanceSamplesResponse, Box<dyn Error>> {
        let response = self.client
            .post(&self.base_url)
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
            .json::<PerformanceSamplesResponse>()
            .await?;

        Ok(response)
    }

    pub async fn get_signatures_for_address(&self, address: &str, limit: u64) -> Result<SignaturesResponse, Box<dyn Error>> {
        let response = self.client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .query(&[("api-key", &self.api_key)])
            .json(&json!({
                "jsonrpc": "2.0",
                "id": "1",
                "method": "getSignaturesForAddress",
                "params": [
                    address,
                    {"limit": limit}
                ]
            }))
            .send()
            .await?
            .json::<SignaturesResponse>()
            .await?;

        Ok(response)
    }
}

// ===============================
// FONCTIONS D'ANALYSE PRINCIPALES
// ===============================

pub async fn analyze_performance_samples(client: &HeliusClient) -> Result<(u64, u64, f64, f64), Box<dyn Error>> {
    println!("📊 Analyse Performance Samples...");
    
    let response = client.get_performance_samples(720).await?;
    let samples = response.result;
    
    if samples.is_empty() {
        return Err("Aucun échantillon reçu".into());
    }
    
    let total_period_secs: u64 = samples.iter().map(|s| s.sample_period_secs).sum();
    let total_hours = total_period_secs as f64 / 3600.0;
    
    let total_transactions: u64 = samples.iter().map(|s| s.num_transactions).sum();
    let total_non_vote: u64 = samples.iter().map(|s| s.num_non_vote_transactions).sum();
    
    let tps_real = if total_period_secs > 0 {
        total_transactions as f64 / total_period_secs as f64
    } else {
        0.0
    };
    
    println!("   ✅ {} échantillons sur {:.1}h", samples.len(), total_hours);
    println!("   ⚡ TPS réel moyen: {:.0}", tps_real);
    println!("   📊 Total observé: {} tx", format_number(total_transactions));
    
    Ok((total_transactions, total_non_vote, tps_real, total_hours))
}

pub async fn quick_contracts_sampling(client: &HeliusClient) -> Result<u64, Box<dyn Error>> {
    println!("🔍 Échantillonnage contrats populaires...");
    
    let major_contracts = vec![
        "So11111111111111111111111111111111111111112", // SOL
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
        "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB", // Jupiter
    ];
    
    let mut total_recent = 0u64;
    
    for contract in major_contracts {
        match client.get_signatures_for_address(contract, 100).await {
            Ok(response) => {
                total_recent += response.result.len() as u64;
            }
            Err(_) => {
                // Continuer même si un contrat échoue
            }
        }
        
        // Petit délai pour éviter le rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
    
    println!("   📦 Échantillon collecté: {} signatures récentes", total_recent);
    Ok(total_recent)
}

// ===============================
// FONCTION PRINCIPALE D'ANALYSE
// ===============================

pub async fn get_solana_metrics() -> Result<SolanaMetrics, Box<dyn Error>> {
    let start_time = std::time::Instant::now();
    let client = HeliusClient::new();

    println!("🚀 Démarrage analyse Solana...");

    // 1. Récupérer les métriques de base
    let total_transactions = match client.get_transaction_count().await {
        Ok(count) => {
            println!("📈 Total historique: {}", format_number(count));
            count
        }
        Err(e) => {
            println!("⚠️ Erreur total transactions: {}", e);
            0
        }
    };

    // 2. Slot actuel
    let current_slot = match client.get_current_slot().await {
        Ok(slot) => {
            println!("🎯 Slot actuel: {}", format_number(slot));
            slot
        }
        Err(e) => {
            println!("⚠️ Erreur slot actuel: {}", e);
            0
        }
    };

    // 3. Analyse de performance
    let (perf_tx, _perf_non_vote, performance_tps, _perf_hours) = 
        match analyze_performance_samples(&client).await {
            Ok(data) => data,
            Err(e) => {
                println!("⚠️ Erreur performance samples: {}", e);
                (0, 0, 0.0, 0.0)
            }
        };

    // 4. Échantillonnage rapide
    let _contracts_sample = match quick_contracts_sampling(&client).await {
        Ok(sample) => sample,
        Err(e) => {
            println!("⚠️ Erreur échantillonnage: {}", e);
            0
        }
    };

    // 5. Calculer le TPS estimé
    let estimated_tps = if performance_tps > 0.0 {
        performance_tps
    } else {
        // Estimation basique si pas de données performance
        3000.0
    };

    // 6. Tokens populaires (simulation pour l'instant)
    let mut token_counts = HashMap::new();
    token_counts.insert("SOL".to_string(), perf_tx / 3);
    token_counts.insert("USDC".to_string(), perf_tx / 5);

    // 7. Plus grosse transaction (simulation)
    let biggest_transaction_sol = 15432.85;
    let biggest_transaction_slot = current_slot.saturating_sub(100);

    let analysis_duration = start_time.elapsed().as_millis() as u64;
    let now = chrono::Utc::now().timestamp();

    println!("✅ Analyse terminée en {}ms", analysis_duration);

    Ok(SolanaMetrics {
        total_transactions,
        current_slot,
        estimated_tps,
        token_counts,
        biggest_transaction_sol,
        biggest_transaction_slot,
        biggest_transaction_time: format_timestamp(now),
        last_update: format_timestamp(now),
        analysis_duration,
    })
}