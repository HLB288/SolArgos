mod data;
mod websocket;

use axum::{Router, response::Html, Json, extract::State};
use tower_http::services::ServeDir;
use std::{env, sync::Arc};
use tokio::sync::Mutex;

// Route API Solana
async fn get_solana_data() -> Json<serde_json::Value> {
    let api_key = env::var("HELIUS_API_KEY")
        .unwrap_or_else(|_| "f3128924-15da-4703-8e52-efba4648eee5".to_string());
    
    match data::solana_client::get_solana_metrics(api_key).await {
        Ok(metrics) => {
            println!("✅ Métriques récupérées");
            
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
            eprintln!("❌ Erreur métriques: {}", e);
            Json(serde_json::json!({
                "status": "error", 
                "message": format!("Erreur API: {}", e)
            }))
        }
    }
}

// Route stats 24h
async fn get_24h_stats() -> Json<serde_json::Value> {
    let api_key = env::var("HELIUS_API_KEY")
        .unwrap_or_else(|_| "f3128924-15da-4703-8e52-efba4648eee5".to_string());
    
    let client = data::solana_client::HeliusClient::new(api_key);
    
    match data::transactions::get_24h_stats(&client).await {
        Ok(stats) => {
            Json(serde_json::json!({
                "status": "success",
                "data": stats
            }))
        }
        Err(e) => {
            eprintln!("❌ Erreur stats 24h: {}", e);
            Json(serde_json::json!({
                "status": "error",
                "message": format!("Erreur: {}", e)
            }))
        }
    }
}

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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    println!("🚀 Démarrage Solana Analytics Dashboard...");
    
    let api_key = env::var("HELIUS_API_KEY")
        .unwrap_or_else(|_| "f3128924-15da-4703-8e52-efba4648eee5".to_string());
    
    println!("✅ Clé API: {}...", &api_key[..8]);
    
    // État WebSocket partagé
    let ws_state = Arc::new(Mutex::new(websocket::WebSocketState::new()));
    
    // Démarrer le listener Helius WebSocket
    let ws_state_clone = ws_state.clone();
    let api_key_clone = api_key.clone();
    tokio::spawn(async move {
            let helius_ws = websocket::HeliusWebSocket::new(api_key_clone);
            loop {
                {
                    let result = helius_ws.start_listening(ws_state_clone.clone()).await;
                    if let Err(e) = result {
                        eprintln!("❌ Erreur WebSocket: {}. Reconnexion dans 5s...", e);
                    }
                } // result est drop ici, avant le sleep
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        });
    
    // Routes
    let app = Router::new()
        .route("/", axum::routing::get(|| async {
            Html(include_str!("../public/index.html"))
        }))
        .route("/api/solana", axum::routing::get(get_solana_data))
        .route("/api/stats24h", axum::routing::get(get_24h_stats))
        .route("/api/health", axum::routing::get(health_check))
        .route("/ws", axum::routing::get(websocket::server::websocket_handler))
        .with_state(ws_state)
        .nest_service("/pkg", ServeDir::new("target/site/pkg"))
        .nest_service("/public", ServeDir::new("public"));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3500));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    println!("🌐 Serveur sur http://{}", addr);
    println!("📊 Dashboard: http://localhost:3500");
    println!("🔌 WebSocket: ws://localhost:3500/ws");
    println!("📈 Stats 24h: http://localhost:3500/api/stats24h");
    println!("\n⏳ Prêt !\n");
    
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    eprintln!("⚠️  Lancez avec: cargo run --features ssr");
}