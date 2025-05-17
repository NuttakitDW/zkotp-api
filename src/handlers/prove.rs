use crate::models::ProveRequest;
use actix_web::{web, HttpResponse};

pub async fn prove_handler(req: web::Json<ProveRequest>) -> HttpResponse {
    println!("📜 Proving request: {:?}", req);
    HttpResponse::Ok().json("proof_abc123")
}
