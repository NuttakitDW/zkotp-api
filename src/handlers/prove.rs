use actix_web::{web, HttpResponse};
use byteorder::{BigEndian, ByteOrder};
use data_encoding::BASE32_NOPAD;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::ProveRequest;

type HmacSha1 = Hmac<Sha1>;

pub async fn prove_handler(req: web::Json<ProveRequest>) -> HttpResponse {
    let secret_bytes: [u8; 32] = match BASE32_NOPAD.decode(req.secret.as_bytes()) {
        Ok(v) => match v.try_into() {
            Ok(arr) => arr,
            Err(_) => return HttpResponse::BadRequest().body("secret must decode to 32 bytes"),
        },
        Err(_) => return HttpResponse::BadRequest().body("invalid base32 secret"),
    };

    let hashed_secret = Sha256::digest(&secret_bytes);

    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let period: u64 = 30;
    let time_step = now_secs / period;

    let mut counter = [0u8; 8];
    BigEndian::write_u64(&mut counter, time_step);

    let mut mac = HmacSha1::new_from_slice(&secret_bytes).unwrap();
    mac.update(&counter);
    let hmac = mac.finalize().into_bytes();
    let hmac_arr: [u8; 20] = hmac.as_slice().try_into().unwrap();

    let msg_json = match serde_json::to_string(&req.message) {
        Ok(s) => s,
        Err(_) => return HttpResponse::BadRequest().body("serialize message failed"),
    };
    let action_hash = Sha256::digest(msg_json.as_bytes());

    let tx_nonce: u32 = req.message.nonce as u32;

    let mut nullifier_hasher = Sha256::new();
    nullifier_hasher.update(&hashed_secret);
    nullifier_hasher.update(&action_hash);
    nullifier_hasher.update(tx_nonce.to_be_bytes());
    let nullifier = nullifier_hasher.finalize();

    let body = serde_json::json!({
        "inputs": {
            "hmac": hex::encode(hmac_arr),
            "hashed_secret": hex::encode(hashed_secret),
            "otp_code": req.otp_code,
            "action_hash": hex::encode(action_hash),
            "tx_nonce": tx_nonce,
            "nullifier": hex::encode(nullifier)
        }
    });

    HttpResponse::Ok().json(body)
}
