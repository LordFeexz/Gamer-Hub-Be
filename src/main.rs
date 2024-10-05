use actix_web::{main, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::{env, io, sync};
mod database;
mod enums;
mod models;

async fn index() -> impl Responder {
    HttpResponse::Ok().body(format!("SUCCESS"))
}

#[main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let client = database::conf::connect_to_db().await;
    let client_data = sync::Arc::new(client);
    let addr = format!(
        "127.0.0.1:{}",
        env::var("PORT").unwrap_or("3001".to_string())
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client_data.clone()))
            .route("/", web::get().to(index))
    })
    .bind(addr)?
    .run()
    .await
}
