// use crate::handlers::{auth, chat, presence};
// use actix_web::web;

// pub fn configure_routes(cfg: &mut web::ServiceConfig) {
//     // cfg.service(web::resource("/register").route(web::post().to(auth::register)))
//     //     .service(web::resource("/login").route(web::post().to(auth::login)))
//     //     .service(web::resource("/logout").route(web::post().to(auth::logout)))
//     //     .service(web::resource("/checkstatus").route(web::post().to(presence::check_status)));
//     // Authentication routes
//     cfg.service(
//         web::scope("/auth") // Routes for authentication
//             .route("/register", web::post().to(auth::register))
//             .route("/login", web::post().to(auth::login))
//             .route("/logout", web::post().to(auth::logout)),
//     )
//     .service(
//         web::scope("/chatroom") // Routes for chat rooms
//             .route("", web::post().to(chat::create_chatroom))
//             .route("/{room_id}/join", web::post().to(chat::join_chatroom))
//             .route("/{room_id}/leave", web::post().to(chat::leave_chatroom))
//             .route(
//                 "/{room_id}/users",
//                 web::get().to(chat::list_users),
//             )
//             .route("/list/{room_id}", web::get().to(chat::list_chatrooms)),
//         // .route("/message", web::post().to(chat::send_message)),
//     );
// }
use crate::handlers::{auth_handler, chat_handler, presence_handler};
use actix_web::web;

use reqwest::Client;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(auth_handler::register))
            .route("/login", web::post().to(auth_handler::login)) // LoginRequest 应派生 Deserialize
            // .route("/logout", web::post().to(auth_handler::logout)), // LogoutRequest 应派生 Deserialize
    )

    // .route("", web::post().to(chat_handler::create_chatroom)) // 改为适配 /chatroom/create
    // .route("/{room_id}/join", web::post().to(chat_handler::join_chatroom))
    // .route("/{room_id}/leave", web::post().to(chat_handler::leave_chatroom))
    // .route("/{room_id}/users", web::get().to(chat_handler::list_users))
    // .route("/list", web::get().to(chat_handler::list_chatrooms))

    .service(
        web::scope("/chatroom")
            .route("/create", web::post().to(chat_handler::create_chatroom)) // 创建聊天室
            .route("/{room_id}/join", web::post().to(chat_handler::join_chatroom)) // 加入聊天室
            .route("/{room_id}/leave", web::post().to(chat_handler::leave_chatroom)) // 离开聊天室
            .route("/{room_id}/users", web::get().to(chat_handler::list_users)) // 查看聊天室用户
            .route("/list", web::get().to(chat_handler::list_chatrooms)), // 查看所有聊天室
    );
}



// list_users

// list_chatrooms

// leave_chatroom

// join_chatroom

// create_chatroom