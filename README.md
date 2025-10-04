Solana Analytics Dashboard

Real-time blockchain analytics platform built entirely in Rust

A production-grade analytics dashboard for the Solana blockchain, featuring live transaction streaming, 24-hour statistics, and comprehensive network metrics via WebSocket and REST API.
Afficher l'image
Afficher l'image
Afficher l'image
Features
Real-Time Data Streaming

Live Transaction Feed: WebSocket connection to Helius RPC streaming SOL, USDC, and Jupiter transactions
Network Activity Monitor: Current TPS (Transactions Per Second) with 5-minute delay for data validation
Auto-Reconnection: Resilient WebSocket connection with automatic retry on failure

Comprehensive Analytics

24-Hour Statistics: Vote vs user transaction breakdown with hourly distribution
Network Health Status: Real-time classification (Excellent/Good/Moderate/Degraded)
Performance Metrics: Current slot, total transactions, estimated TPS
Anomaly Detection: Automated identification of network spikes and drops

Modern Interface

Responsive Dashboard: Clean UI built with Tailwind CSS
Interactive Charts: Real-time graphs using Chart.js
WebSocket Updates: Live data without page refresh
Mobile-Friendly: Optimized for all screen sizes

Quick Start
Prerequisites

Rust 1.70+
Helius API Key (free tier available)

Installation
bash# Clone the repository
git clone https://github.com/yourusername/solana-dashboard.git
cd solana-dashboard

# Configure environment
cp .env.example .env
# Edit .env and add your Helius API key
Build & Run
bash# Development build
cargo build --features ssr

# Run the server
cargo run --features ssr

# Production build (optimized)
cargo build --release --features ssr
./target/release/solana-dashboard
The dashboard will be available at http://localhost:3500
Configuration
Create a .env file in the project root:
envHELIUS_API_KEY=your_helius_api_key_here
RUST_LOG=info
Environment Variables:

HELIUS_API_KEY: Your Helius RPC API key (required)
RUST_LOG: Logging level (debug/info/warn/error)

API Endpoints
REST API
GET /api/solana
Returns current blockchain metrics.
Response:
json{
  "status": "success",
  "total_transactions": 264892147,
  "current_slot": 295841623,
  "estimated_tps": 3042.0,
  "biggest_transaction_sol": 15432.85,
  "network_status": "excellent",
  "last_update": "03/10/2025 11:48:24"
}
GET /api/stats24h
Returns 24-hour transaction statistics.
Response:
json{
  "status": "success",
  "data": {
    "total": 274904873,
    "vote_transactions": 210726543,
    "user_transactions": 64178330,
    "hourly_breakdown": [...],
    "estimated_tps_avg": 3180.6
  }
}
GET /api/health
Health check endpoint.
Response:
json{
  "status": "healthy",
  "service": "Solana Analytics Dashboard",
  "version": "1.0.0"
}
WebSocket
Connect to ws://localhost:3500/ws for real-time updates.
Message Format:
json{
  "timestamp": "2025-10-03T11:48:24Z",
  "tps": 3042.5,
  "recent_transactions": [
    {
      "signature": "5x7...",
      "contract": "SOL",
      "amount_sol": 1234.56,
      "from": "wallet...",
      "to": "wallet...",
      "timestamp": 1696334904
    }
  ]
}
Architecture
┌─────────────────────────────────────────┐
│           Browser Client                │
│  HTML/CSS/JS + WebSocket Client         │
└──────────┬──────────────────────────────┘
           │ HTTP/WS
           │
┌──────────▼──────────────────────────────┐
│        Axum HTTP Server                 │
│  ┌────────────┐    ┌─────────────┐     │
│  │   Routes   │    │  WebSocket  │     │
│  │   /api/*   │    │   Server    │     │
│  └──────┬─────┘    └──────┬──────┘     │
│         │                 │             │
│  ┌──────▼─────────────────▼──────┐     │
│  │    Shared State (Arc<Mutex>)  │     │
│  └──────┬────────────────────────┘     │
│         │                               │
│  ┌──────▼────────────────────┐         │
│  │  Data Processing Layer    │         │
│  │  - Helius RPC Client      │         │
│  │  - Transaction Analysis   │         │
│  │  - 24h Statistics         │         │
│  └──────┬────────────────────┘         │
│         │                               │
│  ┌──────▼───────────────────┐          │
│  │ Helius WebSocket Client  │          │
│  └──────────────────────────┘          │
└─────────────┬───────────────────────────┘
              │ WSS
              │
     ┌────────▼─────────┐
     │   Helius RPC     │
     │  Solana Mainnet  │
     └──────────────────┘
Project Structure
solana-dashboard/
├── src/
│   ├── main.rs              # Server entry point & routes
│   ├── lib.rs               # Module exports
│   ├── data/                # Business logic layer
│   │   ├── mod.rs          # Module orchestration
│   │   ├── models.rs       # Data structures
│   │   ├── solana_client.rs # Helius RPC client
│   │   ├── analysis.rs     # Analytics engine
│   │   └── transactions.rs # 24h statistics
│   └── websocket/           # Real-time layer
│       ├── mod.rs          # WebSocket orchestration
│       ├── helius_stream.rs # Helius WS listener
│       └── server.rs       # Client WS server
├── public/
│   └── index.html          # Frontend application
├── Cargo.toml              # Dependencies
├── .env                    # Configuration (create from .env.example)
└── README.md
Technology Stack
Backend

Axum - Web framework
Tokio - Async runtime
tokio-tungstenite - WebSocket
reqwest - HTTP client
serde - Serialization

Frontend

HTML5 + Tailwind CSS
Vanilla JavaScript
Chart.js
WebSocket API

External Services

Helius - Solana RPC provider

Performance

Memory Usage: ~15-20 MB
WebSocket Latency: <50ms
HTTP Response Time: <100ms
Concurrent Connections: 1000+ clients
CPU Usage: <1% idle, <5% under load

Development
Running Tests
bashcargo test --features ssr
Debug Logging
bashRUST_LOG=debug cargo run --features ssr
Code Formatting
bashcargo fmt
cargo clippy
Deployment
Systemd Service
Create /etc/systemd/system/solana-dashboard.service:
ini[Unit]
Description=Solana Analytics Dashboard
After=network.target

[Service]
Type=simple
User=solana
WorkingDirectory=/opt/solana-dashboard
Environment="HELIUS_API_KEY=your_key_here"
Environment="RUST_LOG=info"
ExecStart=/opt/solana-dashboard/target/release/solana-dashboard
Restart=always

[Install]
WantedBy=multi-user.target
Enable and start:
bashsudo systemctl enable solana-dashboard
sudo systemctl start solana-dashboard
Nginx Reverse Proxy
nginxserver {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://localhost:3500;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
    }
}
Docker (Optional)
dockerfileFROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --features ssr

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/solana-dashboard /usr/local/bin/
COPY --from=builder /app/public /public
CMD ["solana-dashboard"]
Troubleshooting
WebSocket Connection Fails
Error: TLS support not compiled in
Solution: Ensure tokio-tungstenite has native-tls feature:
tomltokio-tungstenite = { version = "0.21", features = ["native-tls"] }
Compilation Errors
bash# Clean build
cargo clean
cargo build --features ssr
No Data Displayed

Check Helius API key is valid
Verify network connectivity
Check server logs: RUST_LOG=debug cargo run --features ssr
Test API directly: curl http://localhost:3500/api/health

Roadmap

 Historical data storage with SurrealDB
 AI-powered whale movement detection (RIG framework)
 LinkedIn automation for daily insights
 Wallet address lookup
 Advanced filtering and search
 Multiple blockchain support
 Docker Compose setup
 Kubernetes deployment configs

Contributing
Contributions are welcome! Please:

Fork the repository
Create a feature branch (git checkout -b feature/amazing-feature)
Commit your changes (git commit -m 'Add amazing feature')
Push to the branch (git push origin feature/amazing-feature)
Open a Pull Request

License
This project is licensed under the MIT License - see the LICENSE file for details.
Acknowledgments

Helius for providing reliable Solana RPC infrastructure
Solana Foundation for the blockchain platform
Rust community for excellent async ecosystem (Tokio, Axum)

Contact
For questions or support, please open an issue on GitHub.

Built with Rust - Zero-cost abstractions, memory safety, and fearless concurrency.
