mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use handlers::{prove, verify};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting API server on http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .route("/verify", web::post().to(verify::verify_handler))
            .route("/prove", web::post().to(prove::prove_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
