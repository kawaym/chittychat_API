use crate::model::db::{User, Session};
use crate::database::setup::DbActor;
use crate::schema::users::dsl::*;
use crate::schema::sessions::dsl::*;
use crate::database::messages::{CreateSession, CreateUser, FetchUser};
use crate::database::insertables::{NewSession, NewUser};

use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: CreateUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");

        let new_user = NewUser {
            nickname: msg.nickname,
            email: msg.email,
            password: msg.password
        };

        diesel::insert_into(users).values(new_user)
        .get_result::<User>(&mut conn)
    }
}

impl Handler<CreateSession> for DbActor {
    type Result = QueryResult<Session>;

    fn handle(&mut self, msg: CreateSession, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Create Session: Unable to create connection");

        let new_session = NewSession {
            hash: msg.hash,
            userid: msg.userid,
        };

        diesel::insert_into(sessions).values(new_session).get_result::<Session>(&mut conn)
    }
}

impl Handler<FetchUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: FetchUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch User: Unable to create connection");

        users.filter(nickname.eq(msg.nickname)).get_result::<User>(&mut conn)
    }
}