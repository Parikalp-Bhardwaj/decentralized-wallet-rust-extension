use serde::{Serialize, Deserialize};


#[derive(Serialize)]
pub struct WalletResponse {
    pub public_key: String,
}

#[derive(Serialize)]
pub struct MnemonicResponse {
    pub mnemonic: String,
}

#[derive(Deserialize)]
pub struct MnemonicRequest{
    pub phrase: String,
    pub passphrase: Option<String>
}

#[derive(Deserialize)]
pub struct BalanceRequest{
    pub public_key: String,
    pub rpc: Option<String>,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub public_key: String,
    pub balance: u64,
}

#[derive(Deserialize)]
pub struct TransferRequest {
    pub sender_private_key: String, 
    pub recipient_public_key: String, 
    pub amount: f64, 
}


#[derive(Serialize)]
pub struct TransferResponse {
    pub transaction_signature: String,
}


#[derive(Deserialize)]
pub struct PrivateKeyRequest {
    pub mnemonic: String,
}

#[derive(Serialize)]
pub struct PrivateKeyRespone {
    pub private_key: String, 
}
