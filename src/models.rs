use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub proof: String,
    pub public_inputs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProveRequest {
    pub secret: String,
    pub message: String,
}
