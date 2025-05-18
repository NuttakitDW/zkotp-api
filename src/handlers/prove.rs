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
    let body = serde_json::json!({});
    HttpResponse::Ok().json(body)
}
