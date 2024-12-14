use crate::models::chat_room::ChatRoom;
use crate::state::SharedState;

// create a chatroom and join it
pub fn create_chatroom(
    state: SharedState,
    room_id: &str,
    max_capacity: usize,
) -> Result<String, String> {
    let mut rooms = state.chat_rooms.lock().unwrap();
    if rooms.contains_key(room_id) {
        return Err(format!("Chatroom {} already exists!", room_id));
    }
    rooms.insert(room_id.to_string(), ChatRoom::new(room_id, max_capacity));
    Ok(format!(
        "Chatroom {} created with capacity {}!",
        room_id, max_capacity
    ))
}

// join a chatroom
pub fn join_chatroom(state: SharedState, room_id: &str, user_id: &str) -> Result<String, String> {
    let mut rooms = state.chat_rooms.lock().unwrap();
    if let Some(room) = rooms.get_mut(room_id) {
        if room.users.len() >= room.max_capacity {
            return Err(format!("Chatroom {} is full!", room_id));
        }

        let user_id_string = user_id.to_string();
        if room.users.contains(&user_id_string) {
            return Ok(format!(
                "User {} is already in chatroom {}!",
                user_id, room_id
            ));
        }

        room.users.push(user_id_string);

        Ok(format!("User {} joined chatroom {}!", user_id, room_id))
    } else {
        Err(format!("Chatroom {} does not exist!", room_id))
    }
}

// leave a chatroom
pub fn leave_chatroom(state: SharedState, room_id: &str, user_id: &str) -> Result<String, String> {
    let mut rooms = state.chat_rooms.lock().unwrap();
    if let Some(room) = rooms.get_mut(room_id) {
        let user_id_string = user_id.to_string();
        if room.users.contains(&user_id_string) {
            room.users.retain(|user| user != &user_id_string);
            Ok(format!("User {} left chatroom {}!", user_id, room_id))
        } else {
            Err(format!("User {} is not in chatroom {}!", user_id, room_id))
        }
    } else {
        Err(format!("Chatroom {} does not exist!", room_id))
    }
}

// list all chat room IDs
pub fn list_chatrooms(state: SharedState) -> Vec<String> {
    let rooms = state.chat_rooms.lock().unwrap();
    rooms.keys().cloned().collect() // return all the chat room ID
}

// list all users in a chat room
pub fn list_users(state: SharedState, room_id: &str) -> Result<Vec<String>, String> {
    let rooms = state.chat_rooms.lock().unwrap();
    if let Some(room) = rooms.get(room_id) {
        Ok(room.users.clone()) // return the user list of the chat room
    } else {
        Err(format!("Chatroom {} does not exist!", room_id))
    }
}
