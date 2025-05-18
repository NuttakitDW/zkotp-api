use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ProveRequest {
    pub otp_code: u32,
    pub secret_b32: String,
    pub tx_msg: String,
}

#[derive(Debug, Serialize)]
pub struct ProveSuccess {
    pub valid: bool,
    pub tx_nonce: u32,
}

#[derive(Debug, Serialize)]
pub struct ProveError {
    pub valid: bool,
    pub reason: String,
}
