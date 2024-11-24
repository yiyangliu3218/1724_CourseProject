use crate::models::user::User;
use crate::models::chat_room::ChatRoom;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// mod services;
// use crate::services::chat_service::ChatServer;

pub struct AppState {
    pub users: Mutex<HashMap<String, User>>, // user id -> User
    pub chat_rooms: Mutex<HashMap<String, ChatRoom>>,  // chatroom id -> chatrooms
}

pub type SharedState = Arc<AppState>;

impl AppState {
    pub fn new() -> SharedState {
        Arc::new(AppState {
            users: Mutex::new(HashMap::new()),
            chat_rooms: Mutex::new(HashMap::new()),
        })
    }
}





