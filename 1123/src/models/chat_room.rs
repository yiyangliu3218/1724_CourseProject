// use tokio::sync::broadcast;

pub struct ChatRoom {
    #[allow(dead_code)]
    pub id: String,                // 聊天室 ID
    pub users: Vec<String>,        // 用户列表
    // pub broadcaster: broadcast::Sender<String>, // 广播通道
    // pub sessions: Vec<Recipient<Message>>, // List of WebSocket session addresses
    pub max_capacity: usize,       // 聊天室最大容量
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

    // pub fn add_user(&mut self, user_id: String, session: Recipient<Message>) {
    //     self.users.push(user_id);
    //     self.sessions.push(session);
    // }

    // pub fn broadcast(&self, message: Message) {
    //     for session in &self.sessions {
    //         let _ = session.do_send(message.clone());
    //     }
    // }
}
