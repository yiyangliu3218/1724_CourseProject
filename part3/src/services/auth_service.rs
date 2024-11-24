use crate::{models::user::User, state::SharedState};
use bcrypt::{hash, verify};

/*Register Function:
  - Check if the user ID valid
  - valid -> add the new user into the HashMap 'users'
  - invalid -> error
*/

pub fn register(state: SharedState, user_id: &str, password: &str) -> Result<String, String> {
    let mut users = state.users.lock().unwrap();
    if users.contains_key(user_id) {
        return Err("User ID already exists.".into());
    }

    let hashed_password = hash(password, 4).map_err(|_| "Password hashing failed.")?;
    let new_user = User {
        id: user_id.to_string(),
        password: hashed_password,
        online: false,
    };

    users.insert(user_id.to_string(), new_user);
    Ok(format!("User {} registered successfully.", user_id))
}

/*Login Function:
  - Check the user ID and password
  - user ID and password match -> change online status
  - Incorrect user ID / password -> error
*/
pub fn login(state: SharedState, user_id: &str, password: &str) -> Result<String, String> {
    let mut users = state.users.lock().unwrap();
    if let Some(user) = users.get_mut(user_id) {
        if verify(password, &user.password).map_err(|_| "Password verification failed.")? {
            user.online = true;
            return Ok(format!("User {} logged in successfully.", user_id));
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
            return Ok(format!("User {} logged out successfully.", user_id));
        } else {
            return Err("User has already logged out.".into());
        }
    }
    Err("User ID not found.".into())
}
