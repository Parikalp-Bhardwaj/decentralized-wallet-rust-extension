use actix_web::{post, get, web, HttpResponse, Responder};
use bip39::Seed;

use bip39::{Mnemonic, MnemonicType, Language};
use crate::account::create_account::Account;
use solana_sdk::{pubkey::Pubkey, signature::{Keypair as SolanaKeypair, Signer},commitment_config::CommitmentConfig};
use solana_client::rpc_client::RpcClient;
use crate::models::models;
use log::{error, info};
use crate::config::Config;
use actix_web::{test, App};
use serde_json::json;


// By calling the generate_mnemonic api we will new mnemonic
#[get("/generate_mnemonic")]
async fn generate_keypair() -> impl Responder{
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let response = models::MnemonicResponse {
        mnemonic: mnemonic.phrase().to_string(),
    };

    info!("Generated new mnemonic");
    HttpResponse::Ok().json(response)
}

// create_wallet is for the creating wallet with mnemonic
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


#[cfg(test)]
mod tests{
    use super::*;
    use actix_web::{test, App, HttpMessage};
    use models::{BalanceRequest, BalanceResponse};
    use serde_json::json;

    #[actix_web::test]
    async fn test_generate_mnemonic(){
        let mut app = test::init_service(
            App::new().service(generate_keypair)
        ).await;

        let req = test::TestRequest::get()
        .uri("/generate_mnemonic")
        .to_request();

        let resp = test::call_service(&mut app, req).await;
        println!("Generate the memonic {:?}",resp.status().is_success());
        assert!(resp.status().is_success());
    }


    #[actix_web::test]
    async fn test_create_wallet(){
        let mnemonic_phrase = "wagon response favorite spoon grace assume upon patrol illness slogan eye planet";

        let mut app = test::init_service(
            App::new().service(create_wallet)
        ).await;


        let response = test::TestRequest::post()
                    .uri("/create_wallet")
                    .set_json(&json!({
                        "phrase": mnemonic_phrase,
                        "passphrase": null  
                    }))
                    .to_request();

        let resp = test::call_service(&mut app, response).await;
        assert!(resp.status().is_success());

        let result: models::WalletResponse = test::read_body_json(resp).await;
        assert!(!result.public_key.is_empty());
        println!("Created wallet with public key: {}", result.public_key);

    }

    #[actix_web::test]
    async fn test_get_balance_rpc_url_override() {
        // Mock Config
        let config = Config {
            solana_rpc_url: "https://api.devnet.solana.com".to_string(),
        };

        // Initialize the app with the get_balance service and a mock Config
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(config))
                .service(get_balance)
        ).await;

        // Define a valid public key to test with
        let public_key = "6QynryR1G6ZV3ztaZtwEnbfCs5fpPiay9hZQqiPEJxaZ";

        // Define a custom RPC URL (for instance, testing on mainnet or a different network)
        let custom_rpc_url = "https://api.mainnet-beta.solana.com";

        // Create a request to the /get_balance endpoint with a custom RPC URL
        let req = test::TestRequest::post()
            .uri("/get_balance")
            .set_json(&json!({
                "public_key": public_key,
                "rpc": custom_rpc_url
            }))
            .to_request();

        // Call the service and get the response
        let resp = test::call_service(&app, req).await;

        // Verify that the response status is a success
        assert!(resp.status().is_success(), "Expected success response status");

        // Deserialize the response body into the BalanceResponse struct
        let result: BalanceResponse = test::read_body_json(resp).await;

        // Verify that the public_key in the response matches the input
        assert_eq!(result.public_key, public_key);
        // Verify that the balance is not zero
        assert!(result.balance == 0);
    }

}
