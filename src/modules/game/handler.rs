use crate::helpers::response::{send_response_body, RespBodyProps};
use crate::modules::game::service;
use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};

pub fn game_handler(conf: &mut web::ServiceConfig) {
    conf.service(web::scope("/game").service(fetch_all_games));
}

#[get("/")]
pub async fn fetch_all_games(data: web::Data<AppState>) -> impl Responder {
    let executor = &data.client;
    let games = service::find_all_games(executor).await;

    let response = RespBodyProps {
        code: 200,
        message: String::from("Success"),
        data: serde_json::to_value(games).unwrap(),
    };

    HttpResponse::Ok().json(send_response_body(response, None))
}
