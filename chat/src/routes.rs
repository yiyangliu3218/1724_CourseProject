use crate::handlers::{auth_handler, presence_handler, chat};

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register").route(web::post().to(auth_handler::register)))
        .service(web::resource("/login").route(web::post().to(auth_handler::login)))
        .service(web::resource("/logout").route(web::post().to(auth_handler::logout)))
        .service(
            web::resource("/checkstatus").route(web::post().to(presence_handler::check_status)),
        )
        // .service(web::resource("/create_chatroom").route(web::post().to(chat::create_chatroom)))
        // .service(web::resource("/join_chatroom").route(web::post().to(chat::join_chatroom)))

        .service(
            web::resource("/chatrooms")
                .route(web::post().to(chat::create_chatroom)) // 创建聊天室
                .route(web::get().to(chat::list_chatrooms)),  // 列出所有聊天室
        )
        .service(
            web::resource("/chatrooms/{room_id}/users")
                .route(web::post().to(chat::join_chatroom))  // 加入聊天室
                .route(web::get().to(chat::list_users_in_chatroom)), // 列出聊天室用户
        )
        .service(
            web::resource("/chatrooms/{room_id}/leave")
                .route(web::post().to(chat::leave_chatroom)), // 离开聊天室
        )
        .service(
            web::resource("/chatrooms/{room_id}/messages")
                .route(web::post().to(chat::send_message)), // 发送消息
        )
        ;
}
