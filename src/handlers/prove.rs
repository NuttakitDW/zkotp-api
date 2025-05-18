use actix_web::{post, web, HttpResponse, Responder};
use tracing::instrument;

use crate::types::{ProveError, ProveRequest, ProveSuccess};

#[post("/prove")]
#[instrument(skip(body))]
pub async fn prove(body: web::Json<ProveRequest>) -> impl Responder {
    let simulated_success = true;

    if simulated_success {
        HttpResponse::Ok().json(ProveSuccess {
            valid: true,
            tx_nonce: 42,
        })
    } else {
        HttpResponse::BadRequest().json(ProveError {
            valid: false,
            reason: "otp mismatch".into(),
        })
    }
}
