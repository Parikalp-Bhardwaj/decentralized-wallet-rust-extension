use bip39::{Mnemonic, Seed, Language};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer};
use solana_sdk::signature::Signer as SolanaSigner; // For Solana Keypair compatibility
use bs58;
use actix_web::{get, post, HttpResponse, Responder,web};
use crate::models::models::{PrivateKeyRequest, PrivateKeyRespone};
use log::{error, info};
pub fn private_key_from_mnemonic(mnemonic_phrase: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mnemonic = Mnemonic::from_phrase(mnemonic_phrase, Language::English)?;

    let seed = Seed::new(&mnemonic, "");

    let secret_key = SecretKey::from_bytes(&seed.as_bytes()[..32])?;

    let public_key = PublicKey::from(&secret_key);

    let mut private_key_bytes = [0u8; 64]; 
    private_key_bytes[..32].copy_from_slice(&secret_key.to_bytes()); 
    private_key_bytes[32..].copy_from_slice(&public_key.to_bytes()); 
    let private_key_base58 = bs58::encode(private_key_bytes).into_string();

    Ok(private_key_base58)
}

#[post("/get_privateKey")]
async fn get_private_key(phrase: web::Json<PrivateKeyRequest>) -> impl Responder{
    let mnemonic = phrase.mnemonic.clone();

    let private_key = match private_key_from_mnemonic(&mnemonic) {
        Ok(private_key) => private_key, 
        Err(err) => {
            error!("Invalid mnemonic phrase provided: {:?}", err);
            return HttpResponse::BadRequest().body(format!("Invalid mnemonic phrase: {:?}", err));
        }
    };
    
    let response = PrivateKeyRespone {        
        private_key,  
    };

    HttpResponse::Ok().json(response)
}