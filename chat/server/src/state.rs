// Use a shared state to store users and chat rooms (to be implemented)
use crate::models::user::User;
use crate::models::user::ChatRoom;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
// use actix_ws::Session;

pub struct AppState {
    pub users: Mutex<HashMap<String, User>>, // user id -> User
                                             // add chat rooms here
    // pub connections: Mutex<HashMap<Session, String>>,  // Session不可以做为Hashmap的键
    pub chat_rooms: Mutex<HashMap<String, ChatRoom>>,  // chatroom id -> chatrooms
}

pub type SharedState = Arc<AppState>;

impl AppState {
    pub fn new() -> SharedState {
        Arc::new(AppState {
            users: Mutex::new(HashMap::new()),
            // add char rooms here
            chat_rooms: Mutex::new(HashMap::new()),
        })
    }
}
