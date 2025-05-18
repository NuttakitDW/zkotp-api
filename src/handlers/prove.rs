use crate::models::ProveRequest;
use actix_web::{web, HttpResponse};
use byteorder::{BigEndian, ByteOrder};
use data_encoding::BASE32_NOPAD;
use hmac::{Hmac, Mac};
use rand::random;
use serde::Deserialize;
use sha1::Sha1;
use sha2::{Digest, Sha256};

type HmacSha1 = Hmac<Sha1>;

pub async fn prove_handler(req: web::Json<ProveRequest>) -> HttpResponse {
    const SECRET_B32: &str = "HNKHWJTBOISS65S6IM2D6UDYNEZCCZLZI5TEWRJJGBJUE33IEVXQ";
    const TIME_STEP: u64 = 0;

    let secret_bytes: [u8; 32] = BASE32_NOPAD
        .decode(SECRET_B32.as_bytes())
        .expect("valid base32")
        .try_into()
        .expect("invalid length");

    let hashed_secret = Sha256::digest(&secret_bytes);
    let mut counter = [0u8; 8];
    BigEndian::write_u64(&mut counter, TIME_STEP);

    let mut mac = HmacSha1::new_from_slice(&secret_bytes).unwrap();
    mac.update(&counter);
    let hmac = mac.finalize().into_bytes();

    let tx_nonce: u32 = random();
    let action_hash = Sha256::digest(req.message.as_bytes());

    let response = serde_json::json!({
        "hmac": hex::encode(hmac),
        "hashed_secret": hex::encode(hashed_secret),
        "otp_code": req.otp_code,
        "action_hash": hex::encode(action_hash),
        "tx_nonce": tx_nonce,
        "mock_proof": "zk-proof-placeholder"
    });

    HttpResponse::Ok().json(response)
}
