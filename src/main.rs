mod api;
mod entities;
mod middleware;
mod models;
mod repositories;

use actix_web::{middleware::Logger, web, App, HttpServer};
use api::{
    health::health_handler,
    post_api::create_post_handler,
    user_api::{create_user_handler, login_user_handler},
};
use sea_orm::DatabaseConnection;

#[derive(Clone, Debug)]
pub struct AppData {
    pub conn: DatabaseConnection,
}

const DATABASE_URL: &str = "postgres://postgres:123456@localhost:5432/rust_database";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let conn = sea_orm::Database::connect(DATABASE_URL).await.unwrap();
    let state = AppData { conn };
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(state.clone()))
            .service(health_handler)
            .service(create_user_handler)
            .service(login_user_handler)
            .configure(post_handler)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

fn post_handler(cfg: &mut web::ServiceConfig) {
    cfg.route("/post", web::route().to(create_post_handler));
}
