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
async fn fetch_all_games(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(send_response_body(
        RespBodyProps {
            code: 200,
            message: String::from("Success"),
            data: Some(serde_json::to_value(service::find_all_games(&data.client).await).unwrap()),
            errors: None,
        },
        None,
    ))
}

#[get("/{id}")]
async fn fetch_game_by_id(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    match service::find_game_by_id(&data.client, path.into_inner()).await {
        Some(game) => HttpResponse::Ok().json(send_response_body(
            RespBodyProps {
                code: 200,
                message: String::from("Success"),
                data: Some(serde_json::to_value(game).unwrap()),
                errors: None,
            },
            None,
        )),
        None => HttpResponse::NotFound().json(send_response_body(
            RespBodyProps {
                code: 404,
                message: String::from("Not Found"),
                data: None,
                errors: None,
            },
            None,
        )),
    }
}
