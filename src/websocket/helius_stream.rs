use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::{TransactionUpdate, RecentTransaction, NetworkActivity};

pub struct HeliusWebSocket {
    url: String,
}

impl HeliusWebSocket {
    pub fn new(api_key: String) -> Self {
        let url = format!("wss://mainnet.helius-rpc.com/?api-key={}", api_key);
        Self { url }
    }

    pub async fn start_listening(
        &self,
        state: Arc<Mutex<super::server::WebSocketState>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔌 Connexion au WebSocket Helius...");

        let (ws_stream, _) = connect_async(&self.url).await?;
        println!("✅ Connecté au WebSocket Helius");

        let (mut write, mut read) = ws_stream.split();

        // S'abonner aux transactions de contrats populaires
        let subscribe_msg = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "transactionSubscribe",
            "params": [
                {
                    "accountInclude": [
                        "So11111111111111111111111111111111111111112",  // SOL
                        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
                        "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB"   // Jupiter
                    ]
                },
                {
                    "commitment": "confirmed",
                    "encoding": "jsonParsed",
                    "transactionDetails": "full",
                    "showRewards": false,
                    "maxSupportedTransactionVersion": 0
                }
            ]
        });

        write.send(Message::Text(subscribe_msg.to_string())).await?;
        println!("📡 Abonné aux transactions...");

        // Traiter les messages
        let mut transaction_count = 0u64;
        let mut last_broadcast = std::time::Instant::now();

        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                        if data.get("method").is_some() {
                            transaction_count += 1;
                            
                            // Parser la transaction
                            if let Some(tx) = self.parse_transaction(&data) {
                                let mut state_guard = state.lock().await;
                                
                                // Garder seulement les 50 dernières
                                if state_guard.recent_transactions.len() >= 50 {
                                    state_guard.recent_transactions.remove(0);
                                }
                                state_guard.recent_transactions.push(tx);
                            }
                        }
                    }

                    // Broadcast toutes les 5 secondes
                    if last_broadcast.elapsed().as_secs() >= 5 {
                        let state_guard = state.lock().await;
                        
                        // Calculer TPS (transactions sur 5 sec)
                        let tps = transaction_count as f64 / 5.0;
                        
                        let update = TransactionUpdate {
                            timestamp: chrono::Utc::now().to_rfc3339(),
                            tps,
                            recent_transactions: state_guard.recent_transactions.clone(),
                        };

                        // Broadcast aux clients
                        super::server::broadcast_to_clients(&state_guard, &update).await;

                        transaction_count = 0;
                        last_broadcast = std::time::Instant::now();
                    }
                }
                Ok(Message::Ping(data)) => {
                    write.send(Message::Pong(data)).await?;
                }
                Ok(Message::Close(_)) => {
                    println!("⚠️ WebSocket fermé par le serveur");
                    break;
                }
                Err(e) => {
                    eprintln!("❌ Erreur WebSocket: {}", e);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn parse_transaction(&self, data: &serde_json::Value) -> Option<RecentTransaction> {
        let params = data.get("params")?;
        let result = params.get("result")?;
        let transaction = result.get("transaction")?;
        
        let signature = result
            .get("signature")
            .and_then(|s| s.as_str())
            .unwrap_or("unknown")
            .to_string();

        let meta = transaction.get("meta")?;
        let pre_balances = meta.get("preBalances")?.as_array()?;
        let post_balances = meta.get("postBalances")?.as_array()?;

        // Calculer le montant transféré
        let mut max_transfer = 0f64;
        for (i, (pre, post)) in pre_balances.iter().zip(post_balances.iter()).enumerate() {
            if let (Some(pre_val), Some(post_val)) = (pre.as_u64(), post.as_u64()) {
                let diff = (post_val as i64 - pre_val as i64).abs() as f64;
                let sol_amount = diff / 1_000_000_000.0;
                if sol_amount > max_transfer {
                    max_transfer = sol_amount;
                }
            }
        }

        // Identifier le contrat
        let contract = self.identify_contract(transaction);

        Some(RecentTransaction {
            signature,
            contract,
            amount_sol: max_transfer,
            from: "wallet...".to_string(), // Simpllifié pour l'instant
            to: "wallet...".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        })
    }

    fn identify_contract(&self, transaction: &serde_json::Value) -> String {
        // Logique simplifiée - à améliorer
        if let Some(message) = transaction.get("message") {
            if let Some(account_keys) = message.get("accountKeys") {
                if let Some(keys) = account_keys.as_array() {
                    for key in keys {
                        if let Some(pubkey) = key.get("pubkey").and_then(|p| p.as_str()) {
                            if pubkey.starts_with("So1111") {
                                return "SOL".to_string();
                            } else if pubkey.starts_with("EPjF") {
                                return "USDC".to_string();
                            } else if pubkey.starts_with("JUP") {
                                return "JUP".to_string();
                            }
                        }
                    }
                }
            }
        }
        "UNKNOWN".to_string()
    }
}