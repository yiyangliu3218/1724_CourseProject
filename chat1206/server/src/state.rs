use crate::models::chat_room::ChatRoom;
// Use a shared state to store users and chat rooms (to be implemented)
use crate::models::user::User;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub users: Mutex<HashMap<String, User>>, // user id -> User
    pub chat_rooms: Mutex<HashMap<String, ChatRoom>>, // room_id -> ChatRoom
                                             // pub sessions: Mutex<HashMap<SocketAddr, String>>, // WebSocket connection -> user id                                       // add chat rooms here
}

pub type SharedState = Arc<AppState>;

impl AppState {
    pub fn new() -> SharedState {
        Arc::new(AppState {
            users: Mutex::new(HashMap::new()),
            chat_rooms: Mutex::new(HashMap::new()),
            // sessions: Mutex::new(HashMap::new()),
        })
    }
}
