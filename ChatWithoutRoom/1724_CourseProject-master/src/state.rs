// Use a shared state to store users and chat rooms (to be implemented)
use crate::models::user::User;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub users: Mutex<HashMap<String, User>>, // user id -> User
                                             // add chat rooms here
}

pub type SharedState = Arc<AppState>;

impl AppState {
    pub fn new() -> SharedState {
        Arc::new(AppState {
            users: Mutex::new(HashMap::new()),
            // add char rooms here
        })
    }
}
