use actix_cors::Cors;
use actix_web::{main, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::{env, io, sync::Arc};
mod database;
mod enums;
mod helpers;
mod models;
mod modules;
mod route;
mod seeders;

pub struct AppState {
    pub client: Arc<sqlx::Pool<sqlx::Postgres>>,
}

#[main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    env_logger::init();

    let client = database::conf::connect_to_db().await;

    let client_data = Arc::new(client);
    let addr = format!(
        "127.0.0.1:{}",
        env::var("PORT").unwrap_or(String::from("3001"))
    );

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default().supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState {
                client: client_data.clone(),
            }))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(route::index_route)
    })
    .bind(addr)?
    .run()
    .await
}
