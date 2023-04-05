use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;


use std::sync::Mutex;
use std::env;
use std::num::ParseIntError;

mod model;
mod routes;
use crate::model::user::User;
struct AppState {
    users: Mutex<Vec<User>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let app_host: String = env::var("APP_HOST").expect("APP_HOST must be set");
    let raw_port: &str = &env::var("PORT").expect("PORT must be set");

    let port:u16;
    
    match raw_port.parse::<u16>() {
        Ok(num) => port = num,
        Err(_) => port = 0000 as u16
    }

    let app_data = web::Data::new(AppState {
        users: Mutex::new(vec![])
    });

    println!("Server running on {}:{}", app_host, port);
    
    HttpServer::new(move || {
        App::new().app_data(app_data.clone())
        .service(routes::index)
    }).bind((app_host, port))?.run().await
}
