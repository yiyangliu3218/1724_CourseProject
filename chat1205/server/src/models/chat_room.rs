use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatRoom {
    pub id: String,             // Room ID
    pub capacity: usize,        // Maximum number of users allowed
    pub users: HashSet<String>, // Set of user IDs in the room
}

impl ChatRoom {
    // Create a new chat room with the give ID and capacity
    pub fn new(id: &str, capacity: usize) -> Self {
        Self {
            id: id.to_string(),
            capacity: capacity,
            users: HashSet::new(),
        }
    }
}
