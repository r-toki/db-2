mod config;
mod controllers;
mod error;
mod models;

use crate::config::CONFIG;

use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let host = &CONFIG.host;
    let port = &CONFIG.port;
    let database_url = &CONFIG.database_url;

    let pool = PgPool::connect(database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(controllers::tasks_controller::init)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
