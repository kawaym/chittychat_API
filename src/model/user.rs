use serde::{Deserialize};

#[derive(Deserialize, Clone)]
pub struct UserCreateData {
    pub nickname: String,
    pub email: String,
    pub password: String,
}
#[derive(Deserialize, Clone)]
pub struct UserLoginData {
    pub nickname: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct UserUpdateData {
    pub password: String,
    pub nickname: String,
}