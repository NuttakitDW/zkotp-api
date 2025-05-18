mod handlers;
mod routes;
mod types;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use tracing_subscriber::EnvFilter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("actix_web=info".parse().unwrap()),
        )
        .init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive()) // tighten later
            .configure(routes::register)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
