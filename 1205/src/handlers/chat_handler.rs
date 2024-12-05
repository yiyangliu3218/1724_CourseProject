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
    data: web::Json<CreateChatRoomRequest>, // 包含 room_id 和 capacity
) -> impl Responder {
    let result = chat_service::create_chatroom(
        state.get_ref().clone(),
        &data.room_id,
        data.capacity,
    )
    .await;
    
    match result {
        Ok(msg) => HttpResponse::Ok().body(msg), // 返回成功消息
        Err(err) => HttpResponse::BadRequest().body(err), // 返回错误消息
    }
}


// #[derive(Deserialize)]
// pub struct JoinChatRoomRequest {
//     pub room_id: String,
//     pub user_id: String,
// }

// /// Join an existing chat room
// pub async fn join_chatroom(
//     state: web::Data<SharedState>,
//     data: web::Json<JoinChatRoomRequest>,
// ) -> impl Responder {
//     let result = chat_service::join_chatroom(
//         state.get_ref().clone(),
//         &data.room_id,
//         &data.user_id,
//     )
//     .await;

//     match result {
//         Ok(_) => HttpResponse::Ok().json(json!({ "message": "Joined chat room successfully" })),
//         Err(e) => HttpResponse::BadRequest().json(json!({ "error": e })),
//     }
// }
#[derive(Deserialize)]
pub struct JoinChatRoomRequest {
    pub room_id: String,
    pub user_id: String,
}

pub async fn join_chatroom(
    state: web::Data<SharedState>,
    data: web::Json<JoinChatRoomRequest>,
) -> impl Responder {
    let result = chat_service::join_chatroom(
        state.get_ref().clone(),
        &data.room_id,
        &data.user_id,
    )
    .await;

    match result {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}


#[derive(Deserialize)]
pub struct LeaveChatRoomRequest {
    pub room_id: String,
    pub user_id: String,
}
pub async fn leave_chatroom(
    state: web::Data<SharedState>,
    data: web::Json<LeaveChatRoomRequest>,
) -> impl Responder {
    let result = chat_service::leave_chatroom(
        state.get_ref().clone(),
        &data.room_id,
        &data.user_id,
    )
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({ "message": "Left chat room successfully" })),
        Err(e) => HttpResponse::BadRequest().json(json!({ "error": e })),
    }
}





/// List all active chat rooms
pub async fn list_chatrooms(state: web::Data<SharedState>) -> impl Responder {
    let rooms = chat_service::list_chatrooms(state.get_ref().clone()).await;

    HttpResponse::Ok().json(json!({
        "chatrooms": rooms
    }))
}

#[derive(Deserialize)]
pub struct ListUsersRequest {
    pub room_id: String,
}

/// List all users in a specific chat room
pub async fn list_users(
    state: web::Data<SharedState>,
    data: web::Json<ListUsersRequest>,
) -> impl Responder {
    let rooms = match state.chat_rooms.lock() {
        Ok(rooms) => rooms,
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": "Failed to acquire lock on chat rooms"
            }));
        }
    };

    if let Some(room) = rooms.get(&data.room_id) {
        let users: Vec<String> = room.users.iter().cloned().collect();
        HttpResponse::Ok().json(json!({ "users": users }))
    } else {
        HttpResponse::NotFound().json(json!({
            "error": format!("Chat room ID '{}' not found", data.room_id)
        }))
    }
}


// use actix_web::{web, HttpResponse, Responder};
// use crate::services::chat_service;
// use crate::state::SharedState;
// use serde::Deserialize;
// use serde_json::json;


// #[derive(Deserialize)]
// pub struct CreateChatRoomRequest {
//     pub room_id: String,
//     pub capacity: usize,
// }

// /// Create a new chat room
// pub async fn create_chatroom(
//     state: web::Data<SharedState>,
//     req: web::Json<CreateChatRoomRequest>,
// ) -> impl Responder {
//     let result = chat_service::create_chatroom(state.get_ref().clone(), &req.room_id, req.capacity).await;

//     match result {
//         Ok(_) => HttpResponse::Ok().json("Chat room created successfully"),
//         Err(e) => HttpResponse::BadRequest().body(format!("Error: {}", e)),
//     }
// }

// /// Join an existing chat room
// pub async fn join_chatroom(
// //     path: web::Path<String>,
//     user_id: String,
// //     state: web::Data<SharedState>,
//     state: web::Data<SharedState>,
//     req: web::Path<String>,
// ) -> impl Responder {
//     let room_id = req.into_inner();
//     let result = chat_service::join_chatroom(state.get_ref().clone(), &room_id,&user_id).await;

//     match result {
//         Ok(_) => HttpResponse::Ok().json("Joined chat room successfully"),
//         Err(e) => HttpResponse::BadRequest().body(format!("Error: {}", e)),
//     }
// }

// /// Leave a chat room
// pub async fn leave_chatroom(
//     state: web::Data<SharedState>,
//     req: web::Path<String>,
//     user_id: String,
// ) -> impl Responder {
//     let room_id = req.into_inner();
//     let result = chat_service::leave_chatroom(state.get_ref().clone(), &room_id,&user_id).await;

//     match result {
//         Ok(_) => HttpResponse::Ok().json("Left chat room successfully"),
//         Err(e) => HttpResponse::BadRequest().body(format!("Error: {}", e)),
//     }
// }

// /// List all active chat rooms
// pub async fn list_chatrooms(state: web::Data<SharedState>) -> impl Responder {
//     let rooms = chat_service::list_chatrooms(state.get_ref().clone()).await;
//     HttpResponse::Ok().json(rooms)
// }


// pub async fn list_users(
//     state: web::Data<SharedState>,
//     room_id: web::Path<String>, // 修改为 Actix Web 的 Path 提供的参数类型
// ) -> impl Responder {
//     let rooms = state.chat_rooms.lock().unwrap();

//     if let Some(room) = rooms.get(&room_id.into_inner()) {
//         let users: Vec<String> = room.users.iter().cloned().collect(); // Collect user IDs into a vector
//         HttpResponse::Ok().json(json!({ "users": users })) // 返回 JSON 响应
//     } else {
//         HttpResponse::NotFound().json(json!({ 
//             "error": format!("Chat room ID is not found")
//         }))
//     }
// }