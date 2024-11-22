/*Check online/offline status */
use crate::state::SharedState;

pub fn check_status(state: SharedState, user_id: &str) -> Result<String, String> {
    let users = state.users.lock().unwrap();
    if let Some(user) = users.get(user_id) {
        return Ok(if user.online {
            format!("User {} is Online", user_id)
        } else {
            format!("User {} is Offline", user_id)
        });
    }
    Err("User ID not found.".into())
}
