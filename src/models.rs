use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub proof: String,
    pub public_inputs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProveRequest {
    pub secret: String,
    pub otp_code: u32,
    pub message: TransactionMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionMessage {
    pub to: String,
    pub value: String,
    pub data: String,
    pub nonce: u64,
    pub chain_id: u64,
    pub action: String,
}
