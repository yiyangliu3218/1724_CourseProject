use crate::handlers::{auth_handler, presence_handler};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register").route(web::post().to(auth_handler::register)))
        .service(web::resource("/login").route(web::post().to(auth_handler::login)))
        .service(web::resource("/logout").route(web::post().to(auth_handler::logout)))
        .service(
            web::resource("/checkstatus").route(web::post().to(presence_handler::check_status)),
        );
}
