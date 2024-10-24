use actix_files as fs;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();  // Initialize the logger
    HttpServer::new(|| App::new()
            .service(fs::Files::new("/", "./public").index_file("index.html")))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}