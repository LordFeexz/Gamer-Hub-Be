use crate::helpers::response::{send_response_body, RespBodyProps};
use crate::modules::game::service;
use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};

pub fn game_handler(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/game")
            .service(fetch_all_games)
            .service(fetch_game_by_id),
    );
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

#[get("/{id}")]
pub async fn fetch_game_by_id(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let executor = &data.client;
    let id = path.into_inner();

    let game = service::find_game_by_id(executor, id).await;
    match game {
        Some(game) => HttpResponse::Ok().json(send_response_body(
            RespBodyProps {
                code: 200,
                message: String::from("Success"),
                data: serde_json::to_value(game).unwrap(),
            },
            None,
        )),
        None => HttpResponse::NotFound().json(send_response_body(
            RespBodyProps {
                code: 404,
                message: String::from("Not Found"),
                data: serde_json::to_value("null").unwrap(),
            },
            None,
        )),
    }
}
