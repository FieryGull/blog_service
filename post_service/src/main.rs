use actix_web::{App, HttpServer};
use std::env;

mod db;
mod posts;
mod error_handler;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_port = env::var("POST_SERVICE_PORT").expect("POST_SERVICE_PORT not found.");
    let app_url = format!("0.0.0.0:{}", &app_port);
    HttpServer::new(|| {
        App::new().configure(posts::init_routes)
    })
        .bind(&app_url)?
        .run()
        .await
}