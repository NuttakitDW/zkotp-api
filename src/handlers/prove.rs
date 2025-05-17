use actix_web::{HttpResponse, web};
use crate::models::ProveRequest;

pub async fn prove_handler(req: web::Json<ProveRequest>) -> HttpResponse {
    println!("📜 Proving request: {:?}", req);
    HttpResponse::Ok().json("proof_abc123")
}
