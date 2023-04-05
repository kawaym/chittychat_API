use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub nickname: String,
    pub email: String,
    pub password: String,
}