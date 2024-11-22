use crate::models::user::User;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// mod services;
use crate::services::chat_service::ChatServer;

pub struct AppState {
    pub users: Mutex<HashMap<String, User>>, // user id -> User
    pub chat_server: Arc<ChatServer>,        // 聊天服务器管理
}

pub type SharedState = Arc<AppState>;

impl AppState {
    pub fn new() -> SharedState {
        Arc::new(AppState {
            users: Mutex::new(HashMap::new()),
            chat_server: ChatServer::new(),
        })
    }
}






