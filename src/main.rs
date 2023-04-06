use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;


use std::sync::Mutex;
use std::env;

mod model;
mod routes;
mod utils;

use crate::{model::{user::User, session::Session}, utils::convert_port_to_u16};
struct AppState {
    users: Mutex<Vec<User>>,
    sessions: Mutex<Vec<Session>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let app_host: String = env::var("APP_HOST").expect("APP_HOST must be set");
    let port: u16 = convert_port_to_u16(&env::var("PORT").expect("PORT must be set"));   
    
    let app_data = web::Data::new(AppState {
        users: Mutex::new(vec![]),
        sessions: Mutex::new(vec![]),
    });

    println!("Server running on {}:{}", app_host, port);
    
    HttpServer::new(move || {
        App::new().app_data(app_data.clone())
        .service(routes::index)
        .configure(routes::user::config)
    }).bind((app_host, port))?.run().await
}
