use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ProveRequest {
    pub otp_code: u32,
    pub secret_b32: String,
    pub tx_msg: String,
}

#[derive(Debug, Serialize)]
pub struct ProveSuccessResponse {
    pub valid: bool,
    pub tx_nouce: u32,
}

#[derive(Debug, Serialize)]
pub struct ProveErrorResponse {
    pub valid: bool,
    pub reason: String,
}
