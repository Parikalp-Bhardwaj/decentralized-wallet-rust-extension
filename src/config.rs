
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub solana_rpc_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        let solana_rpc_url = std::env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());

        Config { solana_rpc_url }
    }
}
