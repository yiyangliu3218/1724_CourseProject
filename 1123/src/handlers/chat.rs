// use actix_web::{web, HttpResponse, Responder};
// use crate::state::SharedState;
// use serde::Deserialize;

// #[derive(Deserialize)]
// pub struct CreateChatRoomRequest {
//     room_id: String,
//     capacity: usize,
// }

// // pub async fn create_chatroom(
// //     data: web::Data<AppState>,
// //     info: web::Json<(String, usize)>, // 从请求体中提取 room_id 和 max_capacity
// // ) -> HttpResponse {
// //     let (room_id, max_capacity) = info.into_inner();
// //     match data.chat_server.create_chatroom(&room_id, max_capacity).await {
// //         Ok(msg) => HttpResponse::Ok().body(msg),
// //         Err(err) => HttpResponse::BadRequest().body(err),
// //     }
// // }

// // pub async fn join_chatroom(
// //     data: web::Data<AppState>,
// //     info: web::Json<(String, String)>, // 从请求体中提取 room_id 和 user_id
// // ) -> HttpResponse {
// //     let (room_id, user_id) = info.into_inner();
// //     match data.chat_server.join_chatroom(&room_id, &user_id).await {
// //         Ok(msg) => HttpResponse::Ok().body(msg),
// //         Err(err) => HttpResponse::BadRequest().body(err),
// //     }
// // }

// // pub async fn leave_chatroom(
// //     data: web::Data<AppState>,
// //     info: web::Json<(String, String)>, // 从请求体中提取 room_id 和 user_id
// // ) -> HttpResponse {
// //     let (room_id, user_id) = info.into_inner();
// //     match data.chat_server.leave_chatroom(&room_id, &user_id).await {
// //         Ok(msg) => HttpResponse::Ok().body(msg),
// //         Err(err) => HttpResponse::BadRequest().body(err),
// //     }
// // }

// // pub async fn send_message(
// //     data: web::Data<AppState>,
// //     info: web::Json<(String, String, String)>, // room_id, user_id, message
// // ) -> HttpResponse {
// //     let (room_id, user_id, message) = info.into_inner();
// //     match data.chat_server.send_message(&room_id, &user_id, &message).await {
// //         Ok(msg) => HttpResponse::Ok().body(msg),
// //         Err(err) => HttpResponse::BadRequest().body(err),
// //     }
// // }

// // pub async fn list_chatrooms(data: web::Data<AppState>) -> HttpResponse {
// //     let chatrooms = data.chat_server.list_chatrooms().await;
// //     HttpResponse::Ok().json(chatrooms) // 返回聊天室 ID 列表
// // }

// // pub async fn list_users_in_chatroom(
// //     data: web::Data<AppState>,
// //     info: web::Path<String>, // room_id
// // ) -> HttpResponse {
// //     let room_id = info.into_inner();
// //     match data.chat_server.list_users_in_chatroom(&room_id).await {
// //         Ok(users) => HttpResponse::Ok().json(users), // 返回用户列表
// //         Err(err) => HttpResponse::BadRequest().body(err),
// //     }
// // }



// pub async fn create_chatroom(
//     data: web::Json<CreateChatRoomRequest>,
//     state: web::Data<SharedState>,
// ) -> impl Responder {
//     let mut chat_rooms = state.chat_rooms.lock().unwrap();

//     if chat_rooms.contains_key(&data.room_id) {
//         return HttpResponse::Conflict().body("Chat room already exists");
//     }

//     chat_rooms.insert(data.room_id.clone(), crate::models::chat_room::ChatRoom {
//         id: data.room_id.clone(),
//         max_capacity: data.capacity,
//         users: vec![],
//     });

//     HttpResponse::Created().body("Chat room created successfully")
// }

// pub async fn join_chatroom(
//     path: web::Path<String>,
//     user_id: String,
//     state: web::Data<SharedState>,
// ) -> impl Responder {
//     let room_id = path.into_inner();
//     let mut chat_rooms = state.chat_rooms.lock().unwrap();

//     if let Some(room) = chat_rooms.get_mut(&room_id) {
//         if room.users.len() >= room.max_capacity {
//             return HttpResponse::Forbidden().body("Room is full");
//         }

//         if !room.users.contains(&user_id) {
//             room.users.push(user_id.clone());
//         }
//         return HttpResponse::Ok().body("Joined chat room");
//     }

//     HttpResponse::NotFound().body("Chat room not found")
// }

// pub async fn leave_chatroom(
//     path: web::Path<String>,
//     user_id: String,
//     state: web::Data<SharedState>,
// ) -> impl Responder {
//     let room_id = path.into_inner();
//     let mut chat_rooms = state.chat_rooms.lock().unwrap();

//     if let Some(room) = chat_rooms.get_mut(&room_id) {
//         room.users.retain(|u| u != &user_id);
//         return HttpResponse::Ok().body("Left chat room");
//     }

//     HttpResponse::NotFound().body("Chat room not found")
// }


// pub async fn list_chatrooms(state: web::Data<SharedState>) -> HttpResponse {
//     let chat_rooms = state.chat_rooms.lock().unwrap();
//     let room_ids: Vec<String> = chat_rooms.keys().cloned().collect();
//     HttpResponse::Ok().json(room_ids) // Return list of chatroom IDs as JSON
// }

// pub async fn list_users(
//     state: web::Data<SharedState>,
//     room_id: web::Path<String>,
// ) -> HttpResponse {
//     let state = state.chat_rooms.lock().unwrap();

//     // Find the chatroom by ID
//     if let Some(chat_room) = state.get(&room_id.into_inner()) {
//         HttpResponse::Ok().json(&chat_room.users) // Return user IDs as JSON
//     } else {
//         HttpResponse::NotFound().body("Chatroom not found")
//     }
// }

use actix_web::{web, HttpResponse, Responder};
use crate::services::chat_service;
use crate::state::SharedState;
use serde::Deserialize;
use serde_json::json;


#[derive(Deserialize)]
pub struct CreateChatRoomRequest {
    pub room_id: String,
    pub capacity: usize,
}

/// Create a new chat room
pub async fn create_chatroom(
    state: web::Data<SharedState>,
    req: web::Json<CreateChatRoomRequest>,
) -> impl Responder {
    let result = chat_service::create_chatroom(state.get_ref().clone(), &req.room_id, req.capacity).await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Chat room created successfully"),
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {}", e)),
    }
}

/// Join an existing chat room
pub async fn join_chatroom(
//     path: web::Path<String>,
    user_id: String,
//     state: web::Data<SharedState>,
    state: web::Data<SharedState>,
    req: web::Path<String>,
) -> impl Responder {
    let room_id = req.into_inner();
    let result = chat_service::join_chatroom(state.get_ref().clone(), &room_id,&user_id).await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Joined chat room successfully"),
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {}", e)),
    }
}

/// Leave a chat room
pub async fn leave_chatroom(
    state: web::Data<SharedState>,
    req: web::Path<String>,
    user_id: String,
) -> impl Responder {
    let room_id = req.into_inner();
    let result = chat_service::leave_chatroom(state.get_ref().clone(), &room_id,&user_id).await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Left chat room successfully"),
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {}", e)),
    }
}

/// List all active chat rooms
pub async fn list_chatrooms(state: web::Data<SharedState>) -> impl Responder {
    let rooms = chat_service::list_chatrooms(state.get_ref().clone()).await;
    HttpResponse::Ok().json(rooms)
}

/// List all users in a specific chat room
// pub async fn list_users(
//     state: web::Data<SharedState>,
//     room_id: &str,
// ) -> Result<Vec<String>, String> {
//     let rooms = state.chat_rooms.lock().unwrap();

//     if let Some(room) = rooms.get(room_id) {
//         let users = room
//             .users
//             .iter()
//             .map(|user_id| user_id.clone()) // Collect user IDs into a vector
//             .collect();
//         Ok(users)
//     } else {
//         Err(format!("Chat room with ID '{}' not found", room_id))
//     }
// }

pub async fn list_users(
    state: web::Data<SharedState>,
    room_id: web::Path<String>, // 修改为 Actix Web 的 Path 提供的参数类型
) -> impl Responder {
    let rooms = state.chat_rooms.lock().unwrap();

    if let Some(room) = rooms.get(&room_id.into_inner()) {
        let users: Vec<String> = room.users.iter().cloned().collect(); // Collect user IDs into a vector
        HttpResponse::Ok().json(json!({ "users": users })) // 返回 JSON 响应
    } else {
        HttpResponse::NotFound().json(json!({ 
            "error": format!("Chat room ID is not found")
        }))
    }
}