use actix_web::{get};

pub mod user;

#[get("/")]
pub async fn index() -> String {
    "This is fine".to_string()
}