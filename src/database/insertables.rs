use crate::schema::{users, sessions};
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub nickname: String,
    pub email: String,
    pub password: String
}

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=sessions)]
pub struct NewSession {
    pub hash: String,
    pub userid: i32,
}
