use bip39::Seed;
use solana_sdk::signature::{Keypair as SolanaKeypair, Signer};
use ed25519_dalek::Keypair;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, commitment_config::CommitmentConfig};

use sha2::{Digest, Sha256};
pub struct Account{
    pub account: SolanaKeypair
}

impl Account{
    pub fn new() -> Self{
        let keypair = SolanaKeypair::new();
        Account { account: keypair }
    }

    pub fn get_public_key(&self) -> String{
        self.account.pubkey().to_string()
    }

    pub fn generate_keypair(seed: &Seed) -> SolanaKeypair{
      
        let private_key_bytes = seed.as_bytes()[..32].to_owned();
        let public_key_bytes = {
            let secret = ed25519_dalek::SecretKey::from_bytes(&private_key_bytes).unwrap();
            let public = ed25519_dalek::PublicKey::from(&secret);
            public.to_bytes()
        };

        let delek_keypair = Keypair{
            secret: ed25519_dalek::SecretKey::from_bytes(&private_key_bytes).unwrap(),
            public: ed25519_dalek::PublicKey::from_bytes(&public_key_bytes).unwrap(),
        };

        SolanaKeypair::from_bytes(&delek_keypair.to_bytes()).unwrap()
    }

    
}


