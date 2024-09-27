use actix_web::{post, get, web, HttpResponse, Responder};
use bip39::Seed;

use bip39::{Mnemonic, MnemonicType, Language};
use crate::account::create_account::Account;
use solana_sdk::{pubkey::Pubkey, signature::{Keypair as SolanaKeypair, Signer},commitment_config::CommitmentConfig};
use solana_client::rpc_client::RpcClient;
use crate::models::models;
use log::{error, info};
use crate::config::Config;


#[get("/generate_mnemonic")]
async fn generate_keypair() -> impl Responder{
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let response = models::MnemonicResponse {
        mnemonic: mnemonic.phrase().to_string(),
    };

    info!("Generated new mnemonic");
    HttpResponse::Ok().json(response)
}


#[post("/create_wallet")]
async fn create_wallet(mnemonic_request: web::Json<models::MnemonicRequest>) -> impl Responder{
    let mnemonic = match Mnemonic::from_phrase(&mnemonic_request.phrase, Language::English){
        Ok(m) => m,
        Err(e) => {
            error!("Invalid mnemonic phrase provided.");
            return HttpResponse::BadRequest().body(format!("Invalid mnemonic phrase: {:?}", e));
        }
    };

    let passphrase = mnemonic_request.passphrase.clone().unwrap_or_default();
    let seed = Seed::new(&mnemonic, &passphrase);

    let keypair = Account::generate_keypair(&seed);
    let pubkey = keypair.pubkey();

    let respone = models::WalletResponse{
        public_key: pubkey.to_string()
    };

    info!("Created wallet with public key: {}", pubkey);

    HttpResponse::Ok().json(respone)
}

#[post("/get_balance")]
async fn get_balance(
    param: web::Json<models::BalanceRequest>,
    config: web::Data<Config>,
) -> impl Responder {
    let pubkey = match param.public_key.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(e) => {
            return HttpResponse::BadRequest().body(format!("Invalid public key: {:?}", e))
        }
    };

    let rpc_url = param.rpc.clone().unwrap_or_else(|| config.solana_rpc_url.clone());

    match fetch_balance(pubkey, rpc_url).await {
        Ok(balance) => {
            let response = models::BalanceResponse {
                public_key: pubkey.to_string(),
                balance,
            };
            info!("Fetched balance for public key: {}", pubkey);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            error!("Error fetching balance: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Error fetching balance: {:?}", e))
        }
    }
}


async fn fetch_balance(
    pubkey: Pubkey,
    rpc_url: String,
) -> Result<u64, Box<dyn std::error::Error>> {
    let result = web::block(move || {
        let client = RpcClient::new_with_commitment(
            rpc_url,
            CommitmentConfig::confirmed(),
        );
        client.get_balance(&pubkey)
    })
    .await
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    Ok(result?)
}

