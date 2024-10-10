use crate::modules::game::handler::game_handler;
use crate::modules::user::handler::user_handler;
use actix_web::web;

pub fn index_route(conf: &mut web::ServiceConfig) {
    conf.service(web::scope("/api/v1").configure(|conf| {
        user_handler(conf);
        game_handler(conf);
    }));
}
