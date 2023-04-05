use actix_web::{post, web, Responder, HttpResponse, put};
use crate::{AppState, User};
use crate::model::user::{CreateUserData, UpdateUserData, LoginUserData};

#[post("/sign-up")]
async fn sign_up(data: web::Data<AppState>, body: web::Json<CreateUserData>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let mut max_id: i32 = 0;
    
    for i in 0..users.len() {
        if users[i].id > max_id {
            max_id = users[i].id
        }
    }

    users.push(User {
        id: max_id + 1,
        nickname: body.nickname.clone(),
        password: body.password.clone(),
        email: body.email.clone(),
    });

    HttpResponse::Created()
}

#[post("/sign-in")]
async fn sign_in(data: web::Data<AppState>, body: web::Json<LoginUserData>) -> impl Responder {
    let users = data.users.lock().unwrap();
    let mut user_exists = -1;
    for i in 0..users.len() {
        if users[i].nickname == body.nickname.clone(){
            user_exists = 0;
            break;
        }
    }
    if user_exists == -1 {
        HttpResponse::NotFound()
    }else {
        
        HttpResponse::NoContent()
    }
}

#[put("/user/{id}")]
async fn update_user_data(data: web::Data<AppState>,path: web::Path<i32>, body: web::Json<UpdateUserData>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let id = path.into_inner();

    for i in 0..users.len() {
        if users[i].id == id {
            users[i].nickname = body.nickname.clone();
            break;
        }
    }

    HttpResponse::NoContent()
}

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(sign_up).service(sign_in).service(update_user_data);
}