use actix_web::{get, post, delete, web, Responder, HttpResponse, App};
use crate::{AppState, User};
use crate::model::user::{CreateUserData};

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