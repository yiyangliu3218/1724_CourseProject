use crate::{models::user::User, state::SharedState};
use actix_web::HttpRequest;
use bcrypt::{hash, verify};
use actix_ws::Session;

/*Register Function:
  - Check if the user ID valid
  - valid -> add the new user into the HashMap 'users'
  - invalid -> error
*/

pub fn register(session:Session, state: SharedState, user_id: &str, password: &str) -> Result<String, String> {
    let mut users = state.users.lock().unwrap();
    if users.contains_key(user_id) {
        println!("User ID already exists.");
        return Err("User ID already exists.".into());
    }

    let hashed_password = hash(password, 4).map_err(|_| "Password hashing failed.")?;
    let new_user = User {
        id: user_id.to_string(),
        password: hashed_password,
        online: false,
        address: String::new(),
        session: session,
    };

    users.insert(user_id.to_string(), new_user);
    // println!("Function correct");
    Ok(format!("User {} registered successfully.", user_id))
}

/*Login Function:
  - Check the user ID and password
  - user ID and password match -> change online status
  - Incorrect user ID / password -> error
*/
pub fn login(session:Session, req: &HttpRequest, state: SharedState, user_id: &str, password: &str) -> Result<String, String> {
    let mut users = state.users.lock().unwrap();
    if let Some(user) = users.get_mut(user_id) {
        // 可能需要每个连接只允许一个user来login
        if verify(password, &user.password).map_err(|_| "Password verification failed.")? {
            if user.online == true {  // 已经登陆
                return Err("User has already logged in.".into());
            }else{
                user.online = true;
                user.session = session.clone();
                if let Some(peer_addr) = req.peer_addr() {
                    let addr = peer_addr.to_string();
                    user.address = addr;
                }
                return Ok(format!("User {} logged in successfully.", user_id));
            }
        } else {
            return Err("Incorrect password.".into());
        }
    }
    Err("User ID not found.".into())
}

/*Logout Function:
  - Check the user ID
  - valid -> change online status
  - invalid -> error
*/
pub fn logout(state: SharedState, user_id: &str) -> Result<String, String> {
    let mut users = state.users.lock().unwrap();
    if let Some(user) = users.get_mut(user_id) {
        if user.online == true {
            user.online = false;
            user.address = String::new();
            return Ok(format!("User {} logged out successfully.", user_id));
        } else {
            return Err("User has already logged out.".into());
        }
    }
    Err("User ID not found.".into())
}
