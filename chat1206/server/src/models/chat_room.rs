// use serde::{Deserialize, Serialize};
// use std::collections::HashSet;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct ChatRoom {
//     pub id: String,             // Room ID
//     pub capacity: usize,        // Maximum number of users allowed
//     pub users: HashSet<String>, // Set of user IDs in the room
// }

// impl ChatRoom {
//     // Create a new chat room with the give ID and capacity
//     pub fn new(id: &str, capacity: usize) -> Self {
//         Self {
//             id: id.to_string(),
//             capacity: capacity,
//             users: HashSet::new(),
//         }
//     }
// }

// use tokio::sync::broadcast;

pub struct ChatRoom {
    #[allow(dead_code)]
    pub id: String, // 聊天室 ID
    pub users: Vec<String>, // 用户列表
    // pub broadcaster: broadcast::Sender<String>, // 广播通道
    // pub sessions: Vec<Recipient<Message>>, // List of WebSocket session addresses
    pub max_capacity: usize, // 聊天室最大容量
}

impl ChatRoom {
    pub fn new(id: &str, max_capacity: usize) -> Self {
        // let (tx, _rx) = broadcast::channel(100);
        Self {
            id: id.to_string(),
            users: Vec::new(),
            // broadcaster: tx,
            max_capacity,
            // sessions: Vec::new(),
        }
    }
}
