use actix_web::{post, web, HttpResponse, Responder};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, pubkey::Pubkey, signer::Signer};
use serde::{Deserialize, Serialize};
use bs58;
use std::thread;
use crate::config::Config;
use crate::models::models::{TransferRequest,TransferResponse};
use base58::FromBase58;

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;


#[post("/send_sol")]
async fn send_sol(req: web::Json<TransferRequest>) -> impl Responder {
    let transfer_result = web::block(move || {
        let sender_keypair_bytes = match bs58::decode(&req.sender_private_key).into_vec() {
            Ok(bytes) => bytes,
            Err(_) => return Err("Invalid sender private key format".to_string()),
        };

        // Ensure the private key is 64 bytes long
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
            Ok(signature) => Ok(signature.to_string()), // Convert the signature to String
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