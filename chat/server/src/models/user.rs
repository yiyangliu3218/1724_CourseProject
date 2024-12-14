// Define the User struct
use actix_ws::Session;

#[derive(Clone)]
// #[derive(Debug)]
pub struct User {
    #[allow(dead_code)]
    pub id: String,
    pub password: String,
    pub online: bool,
    pub session: Session,
}
