mod handlers;
mod routes;
mod types;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .configure(routes::register)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
