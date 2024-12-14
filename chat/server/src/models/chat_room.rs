pub struct ChatRoom {
    #[allow(dead_code)]
    pub id: String, // chat room ID
    pub users: Vec<String>,  // user list of the chat room
    pub max_capacity: usize, // max capacity of the chat room
}

impl ChatRoom {
    pub fn new(id: &str, max_capacity: usize) -> Self {
        Self {
            id: id.to_string(),
            users: Vec::new(),
            max_capacity,
        }
    }
}
