use actix_web::{web, HttpResponse};
use crate::state::AppState;

pub async fn create_chatroom(
    data: web::Data<AppState>,
    info: web::Json<(String, usize)>, // 从请求体中提取 room_id 和 max_capacity
) -> HttpResponse {
    let (room_id, max_capacity) = info.into_inner();
    match data.chat_server.create_chatroom(&room_id, max_capacity).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

pub async fn join_chatroom(
    data: web::Data<AppState>,
    info: web::Json<(String, String)>, // 从请求体中提取 room_id 和 user_id
) -> HttpResponse {
    let (room_id, user_id) = info.into_inner();
    match data.chat_server.join_chatroom(&room_id, &user_id).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

pub async fn leave_chatroom(
    data: web::Data<AppState>,
    info: web::Json<(String, String)>, // 从请求体中提取 room_id 和 user_id
) -> HttpResponse {
    let (room_id, user_id) = info.into_inner();
    match data.chat_server.leave_chatroom(&room_id, &user_id).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

pub async fn send_message(
    data: web::Data<AppState>,
    info: web::Json<(String, String, String)>, // room_id, user_id, message
) -> HttpResponse {
    let (room_id, user_id, message) = info.into_inner();
    match data.chat_server.send_message(&room_id, &user_id, &message).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

pub async fn list_chatrooms(data: web::Data<AppState>) -> HttpResponse {
    let chatrooms = data.chat_server.list_chatrooms().await;
    HttpResponse::Ok().json(chatrooms) // 返回聊天室 ID 列表
}

pub async fn list_users_in_chatroom(
    data: web::Data<AppState>,
    info: web::Path<String>, // room_id
) -> HttpResponse {
    let room_id = info.into_inner();
    match data.chat_server.list_users_in_chatroom(&room_id).await {
        Ok(users) => HttpResponse::Ok().json(users), // 返回用户列表
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
