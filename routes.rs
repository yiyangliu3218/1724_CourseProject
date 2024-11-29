use crate::handlers::{auth, chat, presence};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(web::resource("/register").route(web::post().to(auth::register)))
    //     .service(web::resource("/login").route(web::post().to(auth::login)))
    //     .service(web::resource("/logout").route(web::post().to(auth::logout)))
    //     .service(web::resource("/checkstatus").route(web::post().to(presence::check_status)));
    // Authentication routes
    cfg.service(
        web::scope("/auth") // Routes for authentication
            .route("/register", web::post().to(auth::register))
            .route("/login", web::post().to(auth::login))
            .route("/logout", web::post().to(auth::logout)),
    )
    .service(
        web::scope("/chatroom") // Routes for chat rooms
            .route("", web::post().to(chat::create_chat_room))
            .route("/{room_id}/join", web::post().to(chat::join_chat_room))
            .route("/{room_id}/leave", web::post().to(chat::leave_chat_room))
            .route(
                "/{room_id}/users",
                web::get().to(chat::list_users_in_chat_room),
            )
            .route("/list/{room_id}", web::get().to(chat::list_chat_rooms)),
        // .route("/message", web::post().to(chat::send_message)),
    );
}
