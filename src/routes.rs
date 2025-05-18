use crate::handlers::{health::health, prove::prove};
use actix_web::web;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(health).service(prove);
}
