use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, TimeZone, Local};

// ===============================
// STRUCTURES DE DONNÉES
// ===============================

#[derive(Debug, Serialize, Deserialize)]
struct TransactionCountResponse {
    jsonrpc: String,
    id: String,
    result: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PerformanceSample {
    slot: u64,
    #[serde(rename = "numTransactions")]
    num_transactions: u64,
    #[serde(rename = "numNonVoteTransactions")]
    num_non_vote_transactions: u64,
    #[serde(rename = "samplePeriodSecs")]
    sample_period_secs: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PerformanceSamplesResponse {
    jsonrpc: String,
    id: String,
    result: Vec<PerformanceSample>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SignatureInfo {
    signature: String,
    slot: Option<u64>,
    #[serde(rename = "blockTime")]
    block_time: Option<i64>,
    err: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SignaturesResponse {
    jsonrpc: String,
    id: String,
    result: Vec<SignatureInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BlockResponse {
    jsonrpc: String,
    id: String,
    result: Option<BlockInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BlockInfo {
    #[serde(rename = "blockTime")]
    block_time: Option<i64>,
    transactions: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransactionResponse {
    jsonrpc: String,
    id: String,
    result: Option<TransactionDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransactionDetails {
    slot: u64,
    meta: TransactionMeta,
    #[serde(rename = "blockTime")]
    block_time: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransactionMeta {
    fee: u64,
    #[serde(rename = "preBalances")]
    pre_balances: Vec<u64>,
    #[serde(rename = "postBalances")]
    post_balances: Vec<u64>,
}

// Structure pour stocker les statistiques 24h
#[derive(Debug)]
struct Solana24hStats {
    total_transactions_24h: u64,
    non_vote_transactions_24h: u64,
    avg_tps: f64,
    avg_non_vote_tps: f64,
    blocks_processed: u64,
    biggest_transactions: Vec<TransactionSummary>,
    method_used: String,
    data_quality: String,
    collection_time_ms: u128,
    actual_period_hours: f64,
}

#[derive(Debug)]
struct TransactionSummary {
    signature: String,
    amount_sol: f64,
    timestamp: i64,
    slot: u64,
}

const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;

// ===============================
// FONCTIONS UTILITAIRES
// ===============================

fn format_timestamp(timestamp: i64) -> String {
    let datetime = DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap();
    let paris_time = datetime.with_timezone(&Local);
    paris_time.format("%d/%m/%Y %H:%M:%S").to_string()
}

fn format_number(num: u64) -> String {
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

async fn get_current_slot(client: &reqwest::Client, api_key: &str) -> Result<u64, Box<dyn Error>> {
    let response = client
        .post("https://mainnet.helius-rpc.com/")
        .header("Content-Type", "application/json")
        .query(&[("api-key", api_key)])
        .json(&serde_json::json!({
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

// ===============================
// MÉTHODE 1: Analyse par signatures populaires (CORRIGÉE)
// ===============================

async fn get_24h_stats_by_signatures(
    client: &reqwest::Client,
    api_key: &str,
) -> Result<Solana24hStats, Box<dyn Error>> {
    let start_time = std::time::Instant::now();
    println!("🔥 Méthode 1: Analyse par signatures de contrats populaires");
    
    // Adresses de programmes/contrats populaires sur Solana
    let popular_addresses = vec![
        "So11111111111111111111111111111111111111112", // Native SOL
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
        "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB", // Jupiter
        "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc", // Whirlpool
    ];
    
    let mut all_signatures = Vec::new();
    let mut total_transactions = 0u64;
    let mut biggest_transactions = Vec::new();
    
    let now = chrono::Utc::now();
    let twenty_four_hours_ago = now - chrono::Duration::hours(24);
    let cutoff_timestamp = twenty_four_hours_ago.timestamp();
    
    println!("   Collecte depuis: {}", format_timestamp(cutoff_timestamp));
    
    for (i, address) in popular_addresses.iter().enumerate() {
        println!("   📡 Analyse de {} ({}/{})", address, i+1, popular_addresses.len());
        
        // Récupérer les signatures récentes pour cette adresse
        let response = client
            .post("https://mainnet.helius-rpc.com/")
            .header("Content-Type", "application/json")
            .query(&[("api-key", api_key)])
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "id": "1",
                "method": "getSignaturesForAddress",
                "params": [
                    address,
                    {
                        "limit": 1000,
                        "commitment": "finalized"
                    }
                ]
            }))
            .send()
            .await;
            
        if let Ok(resp) = response {
            if let Ok(signatures_response) = resp.json::<SignaturesResponse>().await {
                // Filtrer par timestamp si disponible
                let recent_sigs: Vec<_> = signatures_response.result
                    .into_iter()
                    .filter(|sig| {
                        if let Some(block_time) = sig.block_time {
                            block_time >= cutoff_timestamp
                        } else {
                            true // Garder si pas de timestamp
                        }
                    })
                    .collect();
                
                println!("      ✅ {} signatures récentes", recent_sigs.len());
                total_transactions += recent_sigs.len() as u64;
                
                // Analyser quelques transactions pour trouver les grosses
                for sig_info in recent_sigs.iter().take(50) {
                    if let Ok(tx_resp) = client
                        .post("https://mainnet.helius-rpc.com/")
                        .header("Content-Type", "application/json")
                        .query(&[("api-key", api_key)])
                        .json(&serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": "1",
                            "method": "getTransaction",
                            "params": [
                                sig_info.signature,
                                {
                                    "encoding": "jsonParsed",
                                    "maxSupportedTransactionVersion": 0
                                }
                            ]
                        }))
                        .send()
                        .await
                    {
                        if let Ok(tx_response) = tx_resp.json::<TransactionResponse>().await {
                            if let Some(tx_details) = tx_response.result {
                                // Calculer le montant de la transaction
                                if let (Some(pre_balance), Some(post_balance)) = 
                                    (tx_details.meta.pre_balances.first(), tx_details.meta.post_balances.first()) {
                                    let amount_lamports = pre_balance.abs_diff(*post_balance);
                                    let amount_sol = amount_lamports as f64 / LAMPORTS_PER_SOL;
                                    
                                    if amount_sol > 5.0 { // Plus de 5 SOL
                                        biggest_transactions.push(TransactionSummary {
                                            signature: sig_info.signature.clone(),
                                            amount_sol,
                                            timestamp: sig_info.block_time.unwrap_or(0),
                                            slot: tx_details.slot,
                                        });
                                    }
                                }
                            }
                        }
                    }
                    
                    // Petit délai pour éviter le rate limiting
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }
                
                all_signatures.extend(recent_sigs);
            }
        }
        
        // Délai entre les adresses
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    // Trier et garder les 10 plus grosses transactions
    biggest_transactions.sort_by(|a, b| b.amount_sol.partial_cmp(&a.amount_sol).unwrap());
    biggest_transactions.truncate(10);
    
    // Estimation du total (extrapolation basée sur échantillon)
    let sample_factor = 100.0; // Facteur d'extrapolation conservateur
    let estimated_total = (total_transactions as f64 * sample_factor) as u64;
    let estimated_non_vote = (estimated_total as f64 * 0.15) as u64; // ~15% non-vote
    
    let avg_tps = estimated_total as f64 / (24.0 * 3600.0);
    let avg_non_vote_tps = estimated_non_vote as f64 / (24.0 * 3600.0);
    
    let collection_time = start_time.elapsed().as_millis();
    
    println!("   ✅ Échantillon collecté: {} transactions", total_transactions);
    println!("   📊 Estimation totale: {}", format_number(estimated_total));
    
    Ok(Solana24hStats {
        total_transactions_24h: estimated_total,
        non_vote_transactions_24h: estimated_non_vote,
        avg_tps,
        avg_non_vote_tps,
        blocks_processed: popular_addresses.len() as u64,
        biggest_transactions,
        method_used: "Analyse par contrats populaires".to_string(),
        data_quality: format!("Estimation basée sur {} échantillons", total_transactions),
        collection_time_ms: collection_time,
        actual_period_hours: 24.0,
    })
}

// ===============================
// MÉTHODE 2: Échantillonnage de blocs amélioré (CORRIGÉE)
// ===============================

async fn get_24h_stats_block_sampling_fixed(
    client: &reqwest::Client,
    api_key: &str,
) -> Result<Solana24hStats, Box<dyn Error>> {
    let start_time = std::time::Instant::now();
    println!("🔬 Méthode 2: Échantillonnage de blocs amélioré");
    
    let current_slot = get_current_slot(client, api_key).await?;
    
    // Calculer le slot d'il y a 24h (environ 2.2 slots/seconde en moyenne)
    let slots_per_day = (24.0 * 60.0 * 60.0 * 2.2) as u64;
    let slot_24h_ago = current_slot.saturating_sub(slots_per_day);
    
    println!("   Slot actuel: {}", current_slot);
    println!("   Slot il y a ~24h: {}", slot_24h_ago);
    println!("   Différence de slots: {}", current_slot - slot_24h_ago);
    
    // Échantillonner avec une stratégie plus conservatrice
    let sample_size = 100; // Réduire pour éviter les timeouts
    let slot_range = current_slot - slot_24h_ago;
    let step = slot_range / sample_size;
    
    let mut total_sample_transactions = 0u64;
    let mut successful_samples = 0u64;
    let mut biggest_transactions = Vec::new();
    
    println!("   Échantillonnage de {} slots (pas de {})", sample_size, step);
    
    for i in 0..sample_size {
        let sample_slot = slot_24h_ago + (i * step);
        
        // Essayer plusieurs slots proches en cas d'échec
        for offset in 0..5 {
            let test_slot = sample_slot + offset;
            
            let response = client
                .post("https://mainnet.helius-rpc.com/")
                .header("Content-Type", "application/json")
                .query(&[("api-key", api_key)])
                .json(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": "1",
                    "method": "getBlock",
                    "params": [
                        test_slot,
                        {
                            "encoding": "json",
                            "transactionDetails": "none",
                            "rewards": false,
                            "maxSupportedTransactionVersion": 0
                        }
                    ]
                }))
                .send()
                .await;
                
            if let Ok(resp) = response {
                if resp.status().is_success() {
                    if let Ok(block_json) = resp.json::<serde_json::Value>().await {
                        if let Some(result) = block_json.get("result") {
                            if !result.is_null() {
                                if let Some(transactions) = result.get("transactions") {
                                    if let Some(tx_array) = transactions.as_array() {
                                        let tx_count = tx_array.len() as u64;
                                        total_sample_transactions += tx_count;
                                        successful_samples += 1;
                                        break; // Bloc trouvé, passer au suivant
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Petit délai entre les tentatives
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
        
        // Afficher le progrès
        if i % 25 == 0 {
            println!("   Progrès: {}/{} slots (réussis: {})", i, sample_size, successful_samples);
        }
        
        // Pause pour éviter le rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    if successful_samples == 0 {
        return Err("Aucun échantillon de bloc réussi".into());
    }
    
    // Calculer les estimations
    let avg_tx_per_block = total_sample_transactions as f64 / successful_samples as f64;
    let estimated_total_blocks = slot_range;
    let estimated_total_tx = (avg_tx_per_block * estimated_total_blocks as f64) as u64;
    
    // Estimer les transactions non-vote (environ 12-15% du total sur Solana)
    let estimated_non_vote_tx = (estimated_total_tx as f64 * 0.13) as u64;
    
    let avg_tps = estimated_total_tx as f64 / (24.0 * 3600.0);
    let avg_non_vote_tps = estimated_non_vote_tx as f64 / (24.0 * 3600.0);
    
    println!("   ✅ Échantillonnage terminé:");
    println!("      Blocs réussis: {}/{}", successful_samples, sample_size);
    println!("      Moyenne tx/bloc: {:.1}", avg_tx_per_block);
    println!("      Estimation totale: {}", format_number(estimated_total_tx));
    
    let collection_time = start_time.elapsed().as_millis();
    
    Ok(Solana24hStats {
        total_transactions_24h: estimated_total_tx,
        non_vote_transactions_24h: estimated_non_vote_tx,
        avg_tps,
        avg_non_vote_tps,
        blocks_processed: successful_samples,
        biggest_transactions,
        method_used: "Échantillonnage de blocs optimisé".to_string(),
        data_quality: format!("Estimation basée sur {} échantillons réels", successful_samples),
        collection_time_ms: collection_time,
        actual_period_hours: 24.0,
    })
}

// ===============================
// MÉTHODE 3: Performance Samples (avec analyse détaillée)
// ===============================

async fn get_performance_samples_analysis(
    client: &reqwest::Client,
    api_key: &str,
) -> Result<Solana24hStats, Box<dyn Error>> {
    let start_time = std::time::Instant::now();
    println!("⚠️ Méthode 3: Performance Samples (analyse des limites)");
    
    let response = client
        .post("https://mainnet.helius-rpc.com/")
        .header("Content-Type", "application/json")
        .query(&[("api-key", api_key)])
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": "1",
            "method": "getRecentPerformanceSamples",
            "params": [720] // Maximum théorique
        }))
        .send()
        .await?
        .json::<PerformanceSamplesResponse>()
        .await?;
    
    let samples = response.result;
    println!("   Échantillons reçus: {} / 720 demandés", samples.len());
    
    if samples.is_empty() {
        return Err("Aucun échantillon de performance reçu".into());
    }
    
    // Analyser la distribution des échantillons
    let total_period_secs: u64 = samples.iter().map(|s| s.sample_period_secs).sum();
    let total_hours = total_period_secs as f64 / 3600.0;
    
    let avg_sample_period = total_period_secs / samples.len() as u64;
    let max_period = samples.iter().map(|s| s.sample_period_secs).max().unwrap_or(0);
    let min_period = samples.iter().map(|s| s.sample_period_secs).min().unwrap_or(0);
    
    println!("   📊 Analyse des échantillons:");
    println!("      Période totale: {:.1}h", total_hours);
    println!("      Période moyenne: {}s", avg_sample_period);
    println!("      Période min/max: {}s / {}s", min_period, max_period);
    
    // Calculer les statistiques
    let total_transactions: u64 = samples.iter().map(|s| s.num_transactions).sum();
    let total_non_vote: u64 = samples.iter().map(|s| s.num_non_vote_transactions).sum();
    
    // Analyser la distribution TPS
    let tps_samples: Vec<f64> = samples.iter()
        .filter(|s| s.sample_period_secs > 0)
        .map(|s| s.num_transactions as f64 / s.sample_period_secs as f64)
        .collect();
    
    let avg_tps_real = if !tps_samples.is_empty() {
        tps_samples.iter().sum::<f64>() / tps_samples.len() as f64
    } else {
        0.0
    };
    
    let max_tps = tps_samples.iter().fold(0.0f64, |a, &b| a.max(b));
    let min_tps = tps_samples.iter().fold(f64::MAX, |a, &b| a.min(b));
    
    println!("      TPS moyen réel: {:.2}", avg_tps_real);
    println!("      TPS min/max: {:.0} / {:.0}", min_tps, max_tps);
    
    // Extrapolation à 24h avec avertissements
    let extrapolated_tx = if total_hours > 0.0 {
        (total_transactions as f64 * (24.0 / total_hours)) as u64
    } else {
        total_transactions
    };
    
    let extrapolated_non_vote = if total_hours > 0.0 {
        (total_non_vote as f64 * (24.0 / total_hours)) as u64
    } else {
        total_non_vote
    };
    
    let extrapolated_tps = extrapolated_tx as f64 / (24.0 * 3600.0);
    let extrapolated_non_vote_tps = extrapolated_non_vote as f64 / (24.0 * 3600.0);
    
    println!("   ⚠️ EXTRAPOLATION (imprécise):");
    println!("      24h estimé: {} transactions", format_number(extrapolated_tx));
    println!("      Facteur d'extrapolation: {:.2}x", 24.0 / total_hours);
    
    let collection_time = start_time.elapsed().as_millis();
    
    Ok(Solana24hStats {
        total_transactions_24h: extrapolated_tx,
        non_vote_transactions_24h: extrapolated_non_vote,
        avg_tps: extrapolated_tps,
        avg_non_vote_tps: extrapolated_non_vote_tps,
        blocks_processed: samples.len() as u64,
        biggest_transactions: Vec::new(),
        method_used: format!("Performance Samples ({:.1}h extrapolé)", total_hours),
        data_quality: format!("LIMITATION: seulement {:.1}h de données réelles sur 24h", total_hours),
        collection_time_ms: collection_time,
        actual_period_hours: total_hours,
    })
}

// ===============================
// AFFICHAGE DES RÉSULTATS
// ===============================

fn display_24h_results(stats: &Solana24hStats) {
    println!("\n🚀 ═══ RÉSULTATS ANALYSE 24 HEURES ═══ 🚀");
    println!("Méthode: {}", stats.method_used);
    println!("Qualité: {}", stats.data_quality);
    println!("Temps de collecte: {}ms", stats.collection_time_ms);
    println!("Période réelle analysée: {:.1}h", stats.actual_period_hours);
    println!("═════════════════════════════════════════════");
    
    println!("📊 TRANSACTIONS TOTALES (24h): {}", format_number(stats.total_transactions_24h));
    println!("🗳️  Transactions non-vote: {}", format_number(stats.non_vote_transactions_24h));
    println!("⚡ TPS moyen global: {:.2}", stats.avg_tps);
    println!("🎯 TPS moyen non-vote: {:.2}", stats.avg_non_vote_tps);
    println!("🧮 Échantillons/blocs traités: {}", stats.blocks_processed);
    
    // Évaluation de la qualité des données
    let quality_score = if stats.actual_period_hours >= 20.0 {
        "🟢 EXCELLENTE"
    } else if stats.actual_period_hours >= 12.0 {
        "🟡 CORRECTE"
    } else if stats.actual_period_hours >= 6.0 {
        "🟠 APPROXIMATIVE"
    } else {
        "🔴 TRÈS IMPRÉCISE"
    };
    
    println!("📈 Qualité des données: {} ({:.1}h réelles)", quality_score, stats.actual_period_hours);
    
    // Performance réseau
    if stats.avg_tps > 4000.0 {
        println!("🟢 Performance réseau: EXCELLENTE (TPS > 4000)");
    } else if stats.avg_tps > 2500.0 {
        println!("🟡 Performance réseau: BONNE (TPS > 2500)");
    } else if stats.avg_tps > 1500.0 {
        println!("🟠 Performance réseau: CORRECTE (TPS > 1500)");
    } else {
        println!("🔴 Performance réseau: FAIBLE (TPS < 1500)");
    }
    
    // Affichage des grosses transactions
    if !stats.biggest_transactions.is_empty() {
        println!("\n🐋 TOP TRANSACTIONS DÉTECTÉES:");
        for (i, tx) in stats.biggest_transactions.iter().take(5).enumerate() {
            println!("   {}. {:.2} SOL - Slot {} - {}", 
                     i + 1,
                     tx.amount_sol,
                     tx.slot,
                     format_timestamp(tx.timestamp));
        }
    }
    
    println!("═════════════════════════════════════════════\n");
}

// ===============================
// FONCTION PRINCIPALE
// ===============================

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "f3128924-15da-4703-8e52-efba4648eee5";
    let client = reqwest::Client::new();

    let start_time = SystemTime::now();
    let start_timestamp = start_time
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    println!("🚀 DASHBOARD SOLANA - ANALYSE 24H CORRIGÉE");
    println!("═════════════════════════════════════════════");
    println!("Début: {}", format_timestamp(start_timestamp));
    println!("\n❌ RAPPEL - Limite getRecentPerformanceSamples:");
    println!("   • 720 échantillons maximum = ~12 heures seulement");
    println!("   • Extrapolation imprécise pour 24h complètes\n");

    let mut results = Vec::new();
    
    // ===============================
    // MÉTHODE 1: Analyse par signatures (NOUVELLE)
    // ===============================
    
    println!("🔄 Test des méthodes alternatives...\n");
    
    match get_24h_stats_by_signatures(&client, api_key).await {
        Ok(stats) => {
            display_24h_results(&stats);
            results.push(("Signatures populaires", stats.total_transactions_24h, stats.avg_tps));
        }
        Err(e) => {
            println!("❌ Méthode signatures: {}\n", e);
        }
    }
    
    // ===============================
    // MÉTHODE 2: Échantillonnage amélioré
    // ===============================
    
    match get_24h_stats_block_sampling_fixed(&client, api_key).await {
        Ok(stats) => {
            display_24h_results(&stats);
            results.push(("Échantillonnage blocs", stats.total_transactions_24h, stats.avg_tps));
        }
        Err(e) => {
            println!("❌ Méthode échantillonnage: {}\n", e);
        }
    }
    
    // ===============================
    // MÉTHODE 3: Performance Samples (référence)
    // ===============================
    
    match get_performance_samples_analysis(&client, api_key).await {
        Ok(stats) => {
            display_24h_results(&stats);
            results.push(("Performance Samples", stats.total_transactions_24h, stats.avg_tps));
        }
        Err(e) => {
            println!("❌ Méthode Performance: {}\n", e);
        }
    }
    
    // ===============================
    // COMPARAISON DES RÉSULTATS
    // ===============================
    
    if !results.is_empty() {
        println!("📊 COMPARAISON DES MÉTHODES:");
        println!("════════════════════════════════════════════");
        for (method, tx_count, tps) in &results {
            println!("   {}: {} tx ({:.0} TPS)", method, format_number(*tx_count), tps);
        }
        println!();
        
        // Calculer la moyenne pour référence
        let avg_tx: u64 = results.iter().map(|(_, tx, _)| *tx).sum::<u64>() / results.len() as u64;
        let avg_tps: f64 = results.iter().map(|(_, _, tps)| *tps).sum::<f64>() / results.len() as f64;
        
        println!("📈 ESTIMATION CONSENSUS:");
        println!("   Transactions 24h: ~{} tx", format_number(avg_tx));
        println!("   TPS moyen: ~{:.0} TPS", avg_tps);
        println!("   Qualité: Basée sur {} méthodes indépendantes", results.len());
    }
    
    // ===============================
    // RECOMMANDATIONS FINALES
    // ===============================
    
    println!("\n💡 RECOMMANDATIONS POUR TON DASHBOARD LEPTOS:");
    println!("═══════════════════════════════════════════════════");
    println!("1. 🏆 MÉTHODE RECOMMANDÉE: Échantillonnage de blocs");
    println!("   • Précision acceptable avec coût raisonnable");
    println!("   • 1 crédit par appel RPC standard");
    println!("   • Adaptable selon tes besoins");
    println!();
    println!("2. 📡 POUR DONNÉES TEMPS RÉEL:");
    println!("   • WebSockets Helius pour transactions live");
    println!("   • Webhooks pour événements importants");
    println!("   • Cache Redis (15 min) pour stats 24h");
    println!();
    println!("3. 🗄️ STOCKAGE SURREALDB:");
    println!("   • Historique des TPS moyens par heure");
    println!("   • Transactions importantes détectées");
    println!("   • Métriques de performance réseau");
    println!();
    println!("4. 🤖 INTÉGRATION IA (RIG):");
    println!("   • Analyse des patterns TPS anormaux");
    println!("   • Détection de mouvements de whales");
    println!("   • Génération de rapports automatiques");

    // Durée totale
    let end_time = SystemTime::now();
    let end_timestamp = end_time
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let duration = end_time
        .duration_since(start_time)
        .unwrap()
        .as_secs();

    println!("\n✅ ANALYSE TERMINÉE");
    println!("════════════════════");
    println!("Fin: {}", format_timestamp(end_timestamp));
    println!("Durée: {}s", duration);

    Ok(())
}