use chrono::{DateTime, Utc};
use serde::{Deserialize};

#[derive(Clone)]
pub struct User {
    pub id: i32,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub date_of_creation: DateTime<Utc>
}

#[derive(Deserialize, Clone)]
pub struct CreateUserData {
    pub nickname: String,
    pub email: String,
    pub password: String,
}
#[derive(Deserialize, Clone)]
pub struct LoginUserData {
    pub nickname: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateUserData {
    pub password: String,
    pub nickname: String,
}