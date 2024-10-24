use actix_web::{post, web, HttpResponse, Responder};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, pubkey::Pubkey, signer::Signer};
use serde::{Deserialize, Serialize};
use bs58;
use std::thread;
use crate::config::Config;
use crate::models::models::{TransferRequest,TransferResponse, PrivateKeyRespone};
use base58::FromBase58;


const LAMPORTS_PER_SOL: u64 = 1_000_000_000;


#[post("/send_sol")]
pub async fn send_sol(req: web::Json<TransferRequest>) -> impl Responder {
    let transfer_result = web::block(move || {
        let sender_keypair_bytes = match bs58::decode(&req.sender_private_key).into_vec() {
            Ok(bytes) => bytes,
            Err(_) => return Err("Invalid sender private key format".to_string()),
        };

        if sender_keypair_bytes.len() != 64 {
            return Err("Invalid sender private key length".to_string());
        }

        let sender_keypair = match Keypair::from_bytes(&sender_keypair_bytes) {
            Ok(keypair) => keypair,
            Err(_) => return Err("Failed to construct keypair".to_string()),
        };


        let recipient_pubkey = match req.recipient_public_key.parse::<Pubkey>() {
            Ok(pubkey) => pubkey,
            Err(_) => return Err("Invalid recipient public key".to_string()),
        };

        let rpc_url = "https://api.devnet.solana.com";
        let rpc_client = RpcClient::new_with_commitment(rpc_url, solana_sdk::commitment_config::CommitmentConfig::confirmed());

        let lamports_to_send = (req.amount * 1_000_000_000.0) as u64;
        let recent_blockhash = match rpc_client.get_latest_blockhash() {
            Ok(blockhash) => blockhash,
            Err(_) => return Err("Failed to get latest blockhash".to_string()),
        };

        let transfer_instruction = solana_sdk::system_instruction::transfer(
            &sender_keypair.pubkey(),
            &recipient_pubkey,
            lamports_to_send,
        );

        let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
            &[transfer_instruction],
            Some(&sender_keypair.pubkey()),
            &[&sender_keypair],
            recent_blockhash,
        );

        match rpc_client.send_and_confirm_transaction(&transaction) {
            Ok(signature) => Ok(signature.to_string()), 
            Err(e) => Err(format!("Transaction failed: {:?}", e)),
        }
    })
    .await;

    match transfer_result {
        Ok(signature) => {
            let response = TransferResponse {
                transaction_signature: signature.unwrap(),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use actix_web::{test, App};
    use serde_json::json;
    use solana_sdk::address_lookup_table::program;
    use crate::account;
    use account::private_key::get_private_key;


    
    #[actix_web::test]
    async fn test_send_sol() {
        // Step 1: Get a private key from the mnemonic
        let mnemonic = "wagon response favorite spoon grace assume upon patrol illness slogan eye planet";

        let mut app = test::init_service(
            App::new()
                .service(get_private_key)
                .service(send_sol)
        ).await;

        // Request to get a private key
        let get_key_req = test::TestRequest::post()
            .uri("/get_privateKey")
            .set_json(&json!({
                "mnemonic": mnemonic
            }))
            .to_request();

        let get_key_resp = test::call_service(&mut app, get_key_req).await;
        assert!(get_key_resp.status().is_success());

        let key_result: PrivateKeyRespone = test::read_body_json(get_key_resp).await;
        let private_key = key_result.private_key;

        // Step 2: Send SOL using the private key
        let recipient_public_key = "8iViKmYRGWffkyGXeQFXBnCHVxSFFXJasCYftMSzaFys";
        let amount = 0.00001; // small amount for testing

        let send_sol_req = test::TestRequest::post()
            .uri("/send_sol")
            .set_json(&json!({
                "sender_private_key": private_key,
                "recipient_public_key": recipient_public_key,
                "amount": amount
            }))
            .to_request();

        let send_sol_resp = test::call_service(&mut app, send_sol_req).await;
        assert!(send_sol_resp.status().is_success(), "Expected successful transaction response");

        // Deserialize the response into TransferResponse struct
        let send_result: TransferResponse = test::read_body_json(send_sol_resp).await;
        assert!(!send_result.transaction_signature.is_empty());
        println!("Transaction Signature: {}", send_result.transaction_signature);
    } 
}