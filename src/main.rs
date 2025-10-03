// // use reqwest;
// // use serde::{Deserialize, Serialize};
// // use std::error::Error;
// // use std::collections::HashMap;
// // use std::time::{SystemTime, UNIX_EPOCH};
// // use chrono::{DateTime, Utc, TimeZone, Local};

// // // ===============================
// // // STRUCTURES OPTIMISÉES
// // // ===============================

// // #[derive(Debug, Serialize, Deserialize)]
// // struct PerformanceSample {
// //     slot: u64,
// //     #[serde(rename = "numTransactions")]
// //     num_transactions: u64,
// //     #[serde(rename = "numNonVoteTransactions")]
// //     num_non_vote_transactions: u64,
// //     #[serde(rename = "samplePeriodSecs")]
// //     sample_period_secs: u64,
// // }

// // #[derive(Debug, Serialize, Deserialize)]
// // struct PerformanceSamplesResponse {
// //     jsonrpc: String,
// //     id: String,
// //     result: Vec<PerformanceSample>,
// // }

// // #[derive(Debug, Serialize, Deserialize)]
// // struct TransactionCountResponse {
// //     jsonrpc: String,
// //     id: String,
// //     result: u64,
// // }

// // #[derive(Debug, Serialize, Deserialize)]
// // struct SignatureInfo {
// //     signature: String,
// //     slot: Option<u64>,
// //     #[serde(rename = "blockTime")]
// //     block_time: Option<i64>,
// //     err: Option<serde_json::Value>,
// // }

// // #[derive(Debug, Serialize, Deserialize)]
// // struct SignaturesResponse {
// //     jsonrpc: String,
// //     id: String,
// //     result: Vec<SignatureInfo>,
// // }

// // #[derive(Debug)]
// // struct SolanaAnalysis {
// //     // Méthode Performance Samples (référence)
// //     performance_total_tx: u64,
// //     performance_non_vote_tx: u64,
// //     performance_tps: f64,
// //     performance_period_hours: f64,
// //     performance_extrapolated_24h: u64,
    
// //     // Méthode globale (validation)
// //     global_total_tx: u64,
    
// //     // Méthodes de validation supplémentaires
// //     top_contracts_sample: u64,
    
// //     // Métriques calculées
// //     final_estimate_24h: u64,
// //     final_tps_avg: f64,
// //     confidence_score: f32,
    
// //     // Métadonnées
// //     analysis_time_ms: u128,
// //     methods_used: Vec<String>,
// // }

// // const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;

// // // ===============================
// // // FONCTIONS UTILITAIRES
// // // ===============================

// // fn format_timestamp(timestamp: i64) -> String {
// //     let datetime = DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap();
// //     let paris_time = datetime.with_timezone(&Local);
// //     paris_time.format("%d/%m/%Y %H:%M:%S").to_string()
// // }

// // fn format_number(num: u64) -> String {
// //     num.to_string()
// //         .chars()
// //         .rev()
// //         .collect::<String>()
// //         .as_bytes()
// //         .chunks(3)
// //         .map(|chunk| std::str::from_utf8(chunk).unwrap())
// //         .collect::<Vec<_>>()
// //         .join(" ")
// //         .chars()
// //         .rev()
// //         .collect::<String>()
// // }

// // async fn get_current_slot(client: &reqwest::Client, api_key: &str) -> Result<u64, Box<dyn Error>> {
// //     let response = client
// //         .post("https://mainnet.helius-rpc.com/")
// //         .header("Content-Type", "application/json")
// //         .query(&[("api-key", api_key)])
// //         .json(&serde_json::json!({
// //             "jsonrpc": "2.0",
// //             "id": "1",
// //             "method": "getSlot",
// //             "params": [{"commitment": "finalized"}]
// //         }))
// //         .send()
// //         .await?
// //         .json::<serde_json::Value>()
// //         .await?;

// //     Ok(response["result"].as_u64().unwrap_or(0))
// // }

// // // ===============================
// // // MÉTHODE PRINCIPALE: Performance Samples (optimisée)
// // // ===============================

// // async fn analyze_performance_samples(
// //     client: &reqwest::Client,
// //     api_key: &str,
// // ) -> Result<(u64, u64, f64, f64), Box<dyn Error>> {
// //     println!("📊 Analyse Performance Samples (méthode principale)");
    
// //     let response = client
// //         .post("https://mainnet.helius-rpc.com/")
// //         .header("Content-Type", "application/json")
// //         .query(&[("api-key", api_key)])
// //         .json(&serde_json::json!({
// //             "jsonrpc": "2.0",
// //             "id": "1",
// //             "method": "getRecentPerformanceSamples",
// //             "params": [720] // Maximum
// //         }))
// //         .send()
// //         .await?
// //         .json::<PerformanceSamplesResponse>()
// //         .await?;
    
// //     let samples = response.result;
    
// //     if samples.is_empty() {
// //         return Err("Aucun échantillon reçu".into());
// //     }
    
// //     // Analyser les données
// //     let total_period_secs: u64 = samples.iter().map(|s| s.sample_period_secs).sum();
// //     let total_hours = total_period_secs as f64 / 3600.0;
    
// //     let total_transactions: u64 = samples.iter().map(|s| s.num_transactions).sum();
// //     let total_non_vote: u64 = samples.iter().map(|s| s.num_non_vote_transactions).sum();
    
// //     // Calculer TPS réel moyen
// //     let tps_real = if total_period_secs > 0 {
// //         total_transactions as f64 / total_period_secs as f64
// //     } else {
// //         0.0
// //     };
    
// //     println!("   ✅ {} échantillons sur {:.1}h", samples.len(), total_hours);
// //     println!("   ⚡ TPS réel moyen: {:.0}", tps_real);
// //     println!("   📊 Total observé: {} tx", format_number(total_transactions));
    
// //     Ok((total_transactions, total_non_vote, tps_real, total_hours))
// // }

// // // ===============================
// // // VALIDATION: Total historique global
// // // ===============================

// // async fn get_global_transaction_count(
// //     client: &reqwest::Client,
// //     api_key: &str,
// // ) -> Result<u64, Box<dyn Error>> {
// //     println!("🌐 Récupération du total historique global");
    
// //     let response = client
// //         .post("https://mainnet.helius-rpc.com/")
// //         .header("Content-Type", "application/json")
// //         .query(&[("api-key", api_key)])
// //         .json(&serde_json::json!({
// //             "jsonrpc": "2.0",
// //             "id": "1",
// //             "method": "getTransactionCount",
// //             "params": [{"commitment": "finalized"}]
// //         }))
// //         .send()
// //         .await?
// //         .json::<TransactionCountResponse>()
// //         .await?;
    
// //     println!("   📈 Total historique: {}", format_number(response.result));
// //     Ok(response.result)
// // }

// // // ===============================
// // // VALIDATION: Échantillonnage rapide de contrats
// // // ===============================

// // async fn quick_contracts_sampling(
// //     client: &reqwest::Client,
// //     api_key: &str,
// // ) -> Result<u64, Box<dyn Error>> {
// //     println!("🔍 Échantillonnage rapide de contrats populaires");
    
// //     // Contrats très populaires sur Solana
// //     let major_contracts = vec![
// //         "So11111111111111111111111111111111111111112", // SOL
// //         "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
// //         "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB", // Jupiter
// //     ];
    
// //     let mut total_recent = 0u64;
    
// //     for contract in major_contracts {
// //         let response = client
// //             .post("https://mainnet.helius-rpc.com/")
// //             .header("Content-Type", "application/json")
// //             .query(&[("api-key", api_key)])
// //             .json(&serde_json::json!({
// //                 "jsonrpc": "2.0",
// //                 "id": "1",
// //                 "method": "getSignaturesForAddress",
// //                 "params": [
// //                     contract,
// //                     {"limit": 100} // Échantillon réduit pour la vitesse
// //                 ]
// //             }))
// //             .send()
// //             .await;
            
// //         if let Ok(resp) = response {
// //             if let Ok(sigs_response) = resp.json::<SignaturesResponse>().await {
// //                 total_recent += sigs_response.result.len() as u64;
// //             }
// //         }
        
// //         // Petit délai
// //         tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
// //     }
    
// //     println!("   📦 Échantillon collecté: {} signatures récentes", total_recent);
// //     Ok(total_recent)
// // }

// // // ===============================
// // // CALCUL DE L'ESTIMATION FINALE
// // // ===============================

// // fn calculate_final_estimate(
// //     performance_tx: u64,
// //     performance_hours: f64,
// //     performance_tps: f64,
// //     global_tx: u64,
// //     contracts_sample: u64,
// // ) -> (u64, f64, f32) {
    
// //     // Méthode 1: Extrapolation Performance Samples
// //     let extrapolated_24h = if performance_hours > 0.0 {
// //         (performance_tx as f64 * (24.0 / performance_hours)) as u64
// //     } else {
// //         performance_tx
// //     };
    
// //     // Méthode 2: Estimation basée sur TPS observé
// //     let tps_based_24h = (performance_tps * 24.0 * 3600.0) as u64;
    
// //     // Méthode 3: Validation par échantillons
// //     // Les 3 contrats représentent environ 5-10% du trafic total
// //     let contracts_estimated = contracts_sample * 200; // Facteur ajusté
    
// //     println!("\n🧮 CALCULS D'ESTIMATION:");
// //     println!("   Extrapolation temporelle: {}", format_number(extrapolated_24h));
// //     println!("   Basée sur TPS observé: {}", format_number(tps_based_24h));
// //     println!("   Validation contrats: {}", format_number(contracts_estimated));
    
// //     // Calcul de la moyenne pondérée
// //     // Performance Samples = poids élevé (données réelles)
// //     // TPS observé = validation
// //     // Contrats = validation supplémentaire
    
// //     let weighted_estimate = (
// //         extrapolated_24h as f64 * 0.6 +  // 60% Performance Samples
// //         tps_based_24h as f64 * 0.3 +     // 30% TPS observé
// //         contracts_estimated as f64 * 0.1  // 10% validation contrats
// //     ) as u64;
    
// //     let final_tps = weighted_estimate as f64 / (24.0 * 3600.0);
    
// //     // Score de confiance basé sur la cohérence des méthodes
// //     let variance = vec![
// //         extrapolated_24h as f64,
// //         tps_based_24h as f64,
// //         contracts_estimated as f64,
// //     ];
    
// //     let mean = variance.iter().sum::<f64>() / variance.len() as f64;
// //     let std_dev = {
// //         let variance_sum: f64 = variance.iter()
// //             .map(|value| {
// //                 let diff = mean - value;
// //                 diff * diff
// //             })
// //             .sum();
// //         (variance_sum / variance.len() as f64).sqrt()
// //     };
    
// //     let coefficient_variation = std_dev / mean;
// //     let confidence = (1.0 - coefficient_variation.min(1.0)).max(0.0) as f32;
    
// //     (weighted_estimate, final_tps, confidence)
// // }

// // // ===============================
// // // AFFICHAGE DES RÉSULTATS
// // // ===============================

// // fn display_final_analysis(analysis: &SolanaAnalysis) {
// //     println!("\n🚀 ═══ ANALYSE SOLANA 24H - RÉSULTATS FINAUX ═══ 🚀");
// //     println!("═══════════════════════════════════════════════════════");
    
// //     println!("📊 DONNÉES DE BASE:");
// //     println!("   Performance Samples: {} tx sur {:.1}h", 
// //              format_number(analysis.performance_total_tx), 
// //              analysis.performance_period_hours);
// //     println!("   TPS réel observé: {:.0}", analysis.performance_tps);
// //     println!("   Non-vote observé: {}", format_number(analysis.performance_non_vote_tx));
// //     println!("   Total historique: {}", format_number(analysis.global_total_tx));
    
// //     println!("\n🎯 ESTIMATION FINALE 24H:");
// //     println!("   Transactions totales: {}", format_number(analysis.final_estimate_24h));
// //     println!("   TPS moyen: {:.0}", analysis.final_tps_avg);
// //     println!("   Confiance: {:.0}%", analysis.confidence_score * 100.0);
    
// //     // Évaluation de performance
// //     let performance_grade = if analysis.final_tps_avg > 4000.0 {
// //         "🟢 EXCELLENTE"
// //     } else if analysis.final_tps_avg > 3000.0 {
// //         "🟡 TRÈS BONNE" 
// //     } else if analysis.final_tps_avg > 2000.0 {
// //         "🟠 BONNE"
// //     } else {
// //         "🔴 MODÉRÉE"
// //     };
    
// //     println!("   Performance réseau: {}", performance_grade);
    
// //     // Contexte et comparaisons
// //     println!("\n📈 CONTEXTE:");
// //     let tx_per_second = analysis.final_estimate_24h / (24 * 3600);
// //     let tx_per_minute = tx_per_second * 60;
// //     let tx_per_hour = tx_per_minute * 60;
    
// //     println!("   Par seconde: ~{} transactions", format_number(tx_per_second));
// //     println!("   Par minute: ~{} transactions", format_number(tx_per_minute));
// //     println!("   Par heure: ~{} transactions", format_number(tx_per_hour));
    
// //     // Répartition estimée
// //     let vote_percentage = ((analysis.final_estimate_24h - (analysis.final_estimate_24h as f64 * 0.15) as u64) as f64 / analysis.final_estimate_24h as f64) * 100.0;
// //     println!("   ~{:.0}% votes de consensus", vote_percentage);
// //     println!("   ~{:.0}% transactions utilisateurs", 100.0 - vote_percentage);
    
// //     println!("\n⏱️ PERFORMANCE ANALYSE:");
// //     println!("   Temps de collecte: {}ms", analysis.analysis_time_ms);
// //     println!("   Méthodes utilisées: {}", analysis.methods_used.join(", "));
    
// //     println!("═══════════════════════════════════════════════════════");
// // }

// // // ===============================
// // // FONCTION PRINCIPALE OPTIMISÉE
// // // ===============================

// // #[tokio::main]
// // async fn main() -> Result<(), Box<dyn Error>> {
// //     let api_key = "f3128924-15da-4703-8e52-efba4648eee5";
// //     let client = reqwest::Client::new();
    
// //     let start_time = std::time::Instant::now();
// //     let start_timestamp = SystemTime::now()
// //         .duration_since(UNIX_EPOCH)
// //         .unwrap()
// //         .as_secs() as i64;

// //     println!("🚀 ANALYSEUR SOLANA 24H - VERSION FINALE OPTIMISÉE");
// //     println!("═══════════════════════════════════════════════════");
// //     println!("Début: {}", format_timestamp(start_timestamp));
// //     println!("🎯 Objectif: Estimation précise des transactions 24h\n");
    
// //     let mut methods_used = Vec::new();
    
// //     // ===============================
// //     // 1. ANALYSE PERFORMANCE SAMPLES (PRINCIPALE)
// //     // ===============================
    
// //     let (perf_tx, perf_non_vote, perf_tps, perf_hours) = 
// //         analyze_performance_samples(&client, api_key).await?;
// //     methods_used.push("Performance Samples".to_string());
    
// //     // ===============================
// //     // 2. TOTAL HISTORIQUE (VALIDATION)
// //     // ===============================
    
// //     let global_tx = match get_global_transaction_count(&client, api_key).await {
// //         Ok(count) => {
// //             methods_used.push("Total Global".to_string());
// //             count
// //         }
// //         Err(e) => {
// //             println!("⚠️ Erreur total global: {}", e);
// //             0
// //         }
// //     };
    
// //     // ===============================
// //     // 3. ÉCHANTILLONNAGE CONTRATS (VALIDATION)
// //     // ===============================
    
// //     let contracts_sample = match quick_contracts_sampling(&client, api_key).await {
// //         Ok(sample) => {
// //             methods_used.push("Échantillonnage Contrats".to_string());
// //             sample
// //         }
// //         Err(e) => {
// //             println!("⚠️ Erreur échantillonnage: {}", e);
// //             0
// //         }
// //     };
    
// //     // ===============================
// //     // 4. CALCUL ESTIMATION FINALE
// //     // ===============================
    
// //     let (final_estimate, final_tps, confidence) = calculate_final_estimate(
// //         perf_tx, perf_hours, perf_tps, global_tx, contracts_sample
// //     );
    
// //     let analysis = SolanaAnalysis {
// //         performance_total_tx: perf_tx,
// //         performance_non_vote_tx: perf_non_vote,
// //         performance_tps: perf_tps,
// //         performance_period_hours: perf_hours,
// //         performance_extrapolated_24h: if perf_hours > 0.0 {
// //             (perf_tx as f64 * (24.0 / perf_hours)) as u64
// //         } else {
// //             perf_tx
// //         },
// //         global_total_tx: global_tx,
// //         top_contracts_sample: contracts_sample,
// //         final_estimate_24h: final_estimate,
// //         final_tps_avg: final_tps,
// //         confidence_score: confidence,
// //         analysis_time_ms: start_time.elapsed().as_millis(),
// //         methods_used,
// //     };
    
// //     // ===============================
// //     // 5. AFFICHAGE RÉSULTATS
// //     // ===============================
    
// //     display_final_analysis(&analysis);
    
// //     // ===============================
// //     // 6. RECOMMANDATIONS POUR TON DASHBOARD
// //     // ===============================
    
// //     println!("\n💡 RECOMMANDATIONS DASHBOARD LEPTOS:");
// //     println!("════════════════════════════════════════════");
// //     println!("1. 📊 MÉTRIQUES PRINCIPALES À AFFICHER:");
// //     println!("   • TPS moyen: ~{:.0}", analysis.final_tps_avg);
// //     println!("   • Transactions/24h: ~{}", format_number(analysis.final_estimate_24h));
// //     println!("   • Confiance données: {:.0}%", confidence * 100.0);
    
// //     println!("\n2. 🔄 STRATÉGIE DE COLLECTE:");
// //     println!("   • Performance Samples toutes les 15 min");
// //     println!("   • Cache SurrealDB pour historique");
// //     println!("   • WebSockets pour temps réel");
    
// //     println!("\n3. 🤖 INTÉGRATION IA (RIG):");
// //     println!("   • Détection anomalies TPS");
// //     println!("   • Prédiction pics d'activité"); 
// //     println!("   • Génération insights LinkedIn");
    
// //     println!("\n4. 📈 INDICATEURS PERFORMANCE:");
// //     if analysis.final_tps_avg > 3000.0 {
// //         println!("   ✅ Réseau Solana en excellente forme");
// //         println!("   🚀 Idéal pour applications haute fréquence");
// //     } else {
// //         println!("   ⚠️ Performance modérée, surveiller");
// //     }
    
// //     let end_timestamp = SystemTime::now()
// //         .duration_since(UNIX_EPOCH)
// //         .unwrap()
// //         .as_secs() as i64;

// //     println!("\n✅ ANALYSE TERMINÉE");
// //     println!("══════════════════");
// //     println!("Fin: {}", format_timestamp(end_timestamp));
// //     println!("Durée: {:.1}s", start_time.elapsed().as_secs_f64());
// //     println!("🎯 Prêt pour intégration dans ton dashboard Rust ! 🚀");

// //     Ok(())
// // }


// // async fn get_solana_data() -> axum::Json<serde_json::Value> {
// //     match solana_dashboard::data::solana_client::get_solana_metrics().await {
// //         Ok(metrics) => axum::Json(serde_json::json!({
// //             "status": "success",
// //             "data": metrics
// //         })),
// //         Err(e) => axum::Json(serde_json::json!({
// //             "status": "error", 
// //             "message": format!("Erreur API Helius: {}", e)
// //         }))
// //     }
// // }

// // #[cfg(feature = "ssr")]
// // #[tokio::main]
// // async fn main() {
// //     use axum::{Router, response::Html, Json};
// //     use tower_http::services::ServeDir;
    
// //     let app = Router::new()
// //         .route("/", axum::routing::get(|| async {
// //             Html(include_str!("../public/index.html"))
// //         }))
// //         .route("/api/solana", axum::routing::get(get_solana_data))
// //         .nest_service("/pkg", ServeDir::new("target/site/pkg"))
// //         .nest_service("/public", ServeDir::new("public"));

// //     let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3500));
// //     let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
// //     println!("Serveur sur http://{}", addr);
    
// //     axum::serve(listener, app.into_make_service()).await.unwrap();
// // }

// // #[cfg(not(feature = "ssr"))]
// // pub fn main() {}

// async fn get_solana_data() -> axum::Json<serde_json::Value> {
//     // Données simulées pour l'instant
//     let data = serde_json::json!({
//         "total_transactions": 264892147,
//         "current_slot": 295841623,
//         "estimated_tps": 3042.0,
//         "biggest_transaction_sol": 15432.85,
//         "status": "success"
//     });
//     axum::Json(data)
// }

// #[cfg(feature = "ssr")]
// #[tokio::main]
// async fn main() {
//     use axum::{Router, response::Html};
//     use tower_http::services::ServeDir;
    
//     let app = Router::new()
//         .route("/", axum::routing::get(|| async {
//             Html(include_str!("../public/index.html"))
//         }))
//         .route("/api/solana", axum::routing::get(get_solana_data))
//         .nest_service("/pkg", ServeDir::new("target/site/pkg"))
//         .nest_service("/public", ServeDir::new("public"));

//     let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3500));
//     let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
//     println!("Serveur sur http://{}", addr);
    
//     axum::serve(listener, app.into_make_service()).await.unwrap();
// }

// #[cfg(not(feature = "ssr"))]
// pub fn main() {}

mod data;

use axum::{Router, response::Html, Json};
use tower_http::services::ServeDir;
use std::env;
use data::get_solana_metrics;
// Route API qui utilise les vraies données Helius
async fn get_solana_data() -> Json<serde_json::Value> {
    // Récupérer la clé API depuis les variables d'environnement
    let api_key = env::var("HELIUS_API_KEY")
        .unwrap_or_else(|_| "f3128924-15da-4703-8e52-efba4648eee5".to_string());
    
    match data::solana_client::get_solana_metrics(api_key).await {
        Ok(metrics) => {
            println!("✅ Métriques récupérées avec succès");
            
            Json(serde_json::json!({
                "status": "success",
                "total_transactions": metrics.total_transactions,
                "current_slot": metrics.current_slot,
                "estimated_tps": metrics.estimated_tps,
                "biggest_transaction_sol": metrics.biggest_transaction_sol,
                "biggest_transaction_slot": metrics.biggest_transaction_slot,
                "biggest_transaction_time": metrics.biggest_transaction_time,
                "performance_samples_count": metrics.performance_samples_count,
                "performance_period_hours": metrics.performance_period_hours,
                "non_vote_transactions": metrics.non_vote_transactions,
                "network_status": metrics.network_status,
                "last_update": metrics.last_update,
                "analysis_duration_ms": metrics.analysis_duration_ms
            }))
        }
        Err(e) => {
            eprintln!("❌ Erreur lors de la récupération des métriques: {}", e);
            
            Json(serde_json::json!({
                "status": "error", 
                "message": format!("Erreur API Helius: {}", e),
                "total_transactions": 0,
                "current_slot": 0,
                "estimated_tps": 0.0,
                "biggest_transaction_sol": 0.0
            }))
        }
    }
}

// Route de santé pour vérifier que le serveur fonctionne
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "Solana Analytics Dashboard",
        "version": "1.0.0"
    }))
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    // Configuration du logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    println!("🚀 Démarrage du serveur Solana Analytics Dashboard...");
    
    // Vérifier la clé API
    match env::var("HELIUS_API_KEY") {
        Ok(key) => println!("✅ Clé API Helius configurée: {}...", &key[..8]),
        Err(_) => println!("⚠️  Utilisation de la clé API par défaut"),
    }
    
    // Configuration des routes
    let app = Router::new()
        .route("/", axum::routing::get(|| async {
            Html(include_str!("../public/index.html"))
        }))
        .route("/api/solana", axum::routing::get(get_solana_data))
        .route("/api/health", axum::routing::get(health_check))
        .nest_service("/pkg", ServeDir::new("target/site/pkg"))
        .nest_service("/public", ServeDir::new("public"));

    // Démarrage du serveur
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3500));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    println!("🌐 Serveur démarré sur http://{}", addr);
    println!("📊 Dashboard accessible sur http://localhost:3500");
    println!("🔌 API endpoint: http://localhost:3500/api/solana");
    println!("❤️  Health check: http://localhost:3500/api/health");
    println!("\n⏳ Prêt à recevoir des requêtes...\n");
    
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    eprintln!("⚠️  Ce binaire nécessite la feature 'ssr'");
    eprintln!("   Lancez avec: cargo run --features ssr");
}