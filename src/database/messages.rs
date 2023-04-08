use crate::model::db::{Session, User};

use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct CreateUser {
    pub nickname: String,
    pub password: String,
    pub email: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct FetchUser {
    pub nickname: String,
    pub password: String,
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Session>")]
pub struct CreateSession {
    pub hash: String,
    pub userid: i32,
}