use actix_web::web;
use crate::handlers;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::health).service(handlers::prove);
}
