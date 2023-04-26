use actix::Addr;
use actix_web::{post, web, Responder, HttpResponse};
use chrono:: Utc;

use crate::database::messages::{CreateUser, FetchUser, CreateSession};
use crate::database::setup::DbActor;
use crate::model::db::User;
use crate::{AppState};
use crate::model::user::{UserCreateData, UserLoginData};

#[post("/create")]
async fn sign_up(data: web::Data<AppState>, body: web::Json<UserCreateData>) -> impl Responder {
    
    let db: Addr<DbActor> = data.as_ref().db.clone();

    match db.send(CreateUser {
        nickname: body.nickname.to_string(),
        email: body.email.to_string(),
        password: body.password.to_string()
    }).await {
        Ok(Ok(info)) => HttpResponse::Created().json(info.nickname),
        _ => HttpResponse::InternalServerError().json("Failed to create article"),
    }
}

#[post("/authenticate")]
async fn sign_in(data: web::Data<AppState>, body: web::Json<UserLoginData>) -> impl Responder {
    let db: Addr<DbActor> = data.as_ref().db.clone();
    let mut user: User = User {id: 0, nickname: "".to_string(), password: "".to_string(), email: "".to_string(), created_at: Utc::now(), updated_at: Utc::now()};
    let user_exists: bool;

    
    match db.send(FetchUser {
        nickname: body.nickname.to_string(),
        password: body.password.to_string(),
        id: 0,
    }).await {
        Ok(Ok(info)) => {user = info; user_exists = true},
        _ => user_exists = false,
    }

    if !user_exists {
        HttpResponse::NotFound().json("User not found");
    }

    match db.send( CreateSession {
        hash: "oie".to_string(),
        userid: user.id,
    }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json({drop(user); info.hash}),
        _ => HttpResponse::InternalServerError().json("Error creating session"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(sign_up).service(sign_in);
}