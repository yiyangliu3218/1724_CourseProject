// Define the User struct
#[derive(Clone)]
pub struct User {
    #[allow(dead_code)]
    pub id: String,
    pub password: String,
    pub online: bool,
}
