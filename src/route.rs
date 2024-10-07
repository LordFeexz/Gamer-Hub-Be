use crate::modules::game::handler::game_handler;
use actix_web::web;

pub fn index_route(conf: &mut web::ServiceConfig) {
    conf.service(web::scope("/api/v1").configure(game_handler));
}
