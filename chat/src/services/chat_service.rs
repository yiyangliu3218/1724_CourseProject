use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::chat_room::ChatRoom;

pub struct ChatServer {
    pub rooms: Mutex<HashMap<String, ChatRoom>>,
}

impl ChatServer {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            rooms: Mutex::new(HashMap::new()),
        })
    }

    pub async fn create_chatroom(&self, room_id: &str, max_capacity: usize) -> Result<String, String> {
        let mut rooms = self.rooms.lock().await;
        if rooms.contains_key(room_id) {
            return Err(format!("Chatroom {} already exists!", room_id));
        }
        rooms.insert(room_id.to_string(), ChatRoom::new(room_id, max_capacity));
        Ok(format!("Chatroom {} created with capacity {}!", room_id, max_capacity))
    }

    pub async fn join_chatroom(&self, room_id: &str, user_id: &str) -> Result<String, String> {
        let mut rooms = self.rooms.lock().await;
        if let Some(room) = rooms.get_mut(room_id) {
            if room.users.len() >= room.max_capacity {
                return Err(format!("Chatroom {} is full!", room_id));
            }
    
            // 将 user_id 转换为 String 的引用
            let user_id_string = user_id.to_string();
            if room.users.contains(&user_id_string) {
                return Ok(format!("User {} is already in chatroom {}!", user_id, room_id));
            }
    
            room.users.push(user_id_string);
    
            let mut rx = room.broadcaster.subscribe();
            let room_id_cloned = room_id.to_string(); // 克隆 room_id
            tokio::spawn(async move {
                while let Ok(message) = rx.recv().await {
                    println!("[{}]: {}", room_id_cloned, message);
                }
            });
    
            Ok(format!("User {} joined chatroom {}!", user_id, room_id))
        } else {
            Err(format!("Chatroom {} does not exist!", room_id))
        }
    }
    
    pub async fn leave_chatroom(&self, room_id: &str, user_id: &str) -> Result<String, String> {
        let mut rooms = self.rooms.lock().await;
        if let Some(room) = rooms.get_mut(room_id) {
            let user_id_string = user_id.to_string();
            if room.users.contains(&user_id_string) {
                room.users.retain(|user| user != &user_id_string); // 从用户列表移除用户
                Ok(format!("User {} left chatroom {}!", user_id, room_id))
            } else {
                Err(format!("User {} is not in chatroom {}!", user_id, room_id))
            }
        } else {
            Err(format!("Chatroom {} does not exist!", room_id))
        }
    }

    pub async fn send_message(&self, room_id: &str, user_id: &str, message: &str) -> Result<String, String> {
        let rooms = self.rooms.lock().await;
        if let Some(room) = rooms.get(room_id) {
            let full_message = format!("{}: {}", user_id, message);
            let _ = room.broadcaster.send(full_message);
            Ok(format!("Message sent to chatroom {}!", room_id))
        } else {
            Err(format!("Chatroom {} does not exist!", room_id))
        }
    }

    pub async fn list_chatrooms(&self) -> Vec<String> {
        let rooms = self.rooms.lock().await;
        rooms.keys().cloned().collect() // 返回所有聊天室 ID
    }

    pub async fn list_users_in_chatroom(&self, room_id: &str) -> Result<Vec<String>, String> {
        let rooms = self.rooms.lock().await;
        if let Some(room) = rooms.get(room_id) {
            Ok(room.users.clone()) // 返回用户列表
        } else {
            Err(format!("Chatroom {} does not exist!", room_id))
        }
    }
    
}