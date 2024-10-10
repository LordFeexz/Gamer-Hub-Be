use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
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
mod thirdparty;

pub struct AppState {
    pub client: Arc<sqlx::Pool<sqlx::Postgres>>,
    pub jwt_handler: Arc<helpers::jwt::Jwt>,
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
    let jwt_handler = Arc::new(helpers::jwt::Jwt::new());
    let addr = format!(
        "127.0.0.1:{}",
        env::var("PORT").unwrap_or(String::from("3001"))
    );
    let global_limiter_conf = GovernorConfigBuilder::default()
        .requests_per_minute(1000)
        .burst_size(100)
        .finish()
        .unwrap();

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default().supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState {
                client: client_data.clone(),
                jwt_handler: jwt_handler.clone(),
            }))
            .wrap(Governor::new(&global_limiter_conf))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(route::index_route)
    })
    .bind(addr)?
    .run()
    .await
}
