use actix_web::{HttpResponse, web};
use crate::models::VerifyRequest;

pub async fn verify_handler(req: web::Json<VerifyRequest>) -> HttpResponse {
    println!("🛂 Verifying request: {:?}", req);
    HttpResponse::Ok().json(true)
}
