mod data;

use axum::{Router, response::Html, Json};
use tower_http::services::ServeDir;
use std::env;
use data::get_solana_metrics;

async fn get_solana_data() -> Json<serde_json::Value> {
    // Récupérer la clé API depuis les variables d'environnement
    let api_key = match env::var("HELIUS_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("❌ La variable d'environnement HELIUS_API_KEY n'est pas définie.");
            return Json(serde_json::json!({
                "status": "error",
                "message": "La clé API Helius est manquante.",
                "total_transactions": 0,
                "current_slot": 0,
                "estimated_tps": 0.0,
                "biggest_transaction_sol": 0.0
            }));
        }
    };

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