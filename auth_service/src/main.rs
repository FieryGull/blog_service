extern crate core;

use actix_web::{middleware::Logger, App, HttpServer};
use std::env;

mod common_lib;
mod schema;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let app_port = env::var("AUTH_SERVICE_PORT").expect("AUTH_SERVICE_PORT not found.");
    let app_url = format!("0.0.0.0:{}", &app_port);
    HttpServer::new(|| {
        App::new()
            .configure(users::init_routes)
            .wrap(Logger::new("%a %r %s %T"))
    })
    .bind(&app_url)?
    .run()
    .await
}
