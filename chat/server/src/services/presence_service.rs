/*Check online/offline status */
use crate::state::SharedState;

pub fn check_status(state: SharedState, user_id: &str) -> Result<String, String> {
    let users = state.users.lock().unwrap();
    let userid = user_id.trim().to_string();
    if let Some(user) = users.get(&userid) {
        return Ok(if user.online {
            // println!("Online");
            format!("User {} is Online", user_id)
        } else {
            // println!("Offline");
            format!("User {} is Offline", user_id)
        });
    }
    Err("User ID not found.".into())
}

pub async fn private_chat(state: SharedState, receiver: &str, message: &str, sender: &str) -> Result<String, String>{
    let users = state.users.lock().unwrap();
    let receiver1 = receiver.trim().to_string();
    if let Some(user) = users.get(&receiver1) {
        return Ok(if user.online {
            let mut target = user.session.clone();
            let packet = format!("[Private Chat][{}]:\n{}", sender, message);
            match target.text(packet).await {
                Ok(()) => {
                    // println!("Message sent successfully.");
                    format!("Message has been sent.")
                },
                Err(e) => {
                    // println!("Failed to send due to error: {}", e);
                    format!("{}", e)
                },
            }
        } else {
            // println!("User is Offline");
            format!("User {} is Offline", receiver)
        });
    }
    Err("User ID not found.".into())    
}

pub async fn send_message(state: SharedState, target_room: &str, message: &str, sender: &str) -> Result<String, String>{
    let users = state.users.lock().unwrap();
    let rooms = state.chat_rooms.lock().unwrap();
    let target_room = target_room.trim().to_string();
    if let Some(room) = rooms.get(&target_room) {
        let mut in_room = false;
        let room_users = room.users.clone();
        for room_user in room_users.clone(){
            if sender.to_string()==room_user{
                in_room = true;
            }
        }
        if in_room{
            for room_user in room_users{
                if let Some(user) = users.get(&room_user){
                    if user.online {
                        let mut target = user.session.clone();
                        let packet = format!("[Room:{}][{}]:\n{}", target_room, sender, message);
                        target.text(packet).await.unwrap();
                    }
                }
            }
            return Ok(format!("Message has been sent."));
        }else{
            return Err(format!("You have to join the room first to send message!"));
        }
    }else {
        Err(format!("Chatroom {} does not exist!", target_room))
    }
}