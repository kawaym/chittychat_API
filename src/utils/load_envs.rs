use std::env;
use dotenvy;

use crate::utils::convert_port_to_u16;

pub struct EnvFile {
    pub port: u16,
    pub database_url: String,
    pub app_host: String,
}

pub fn load() -> EnvFile{
    //! Load all variables from .env file and creates an object containing everything in snake_case
    let env: String;
    match env::var("APP_ENV").ok() {
        Some(info) => env = info,
        None => env = "development".to_string(), 
    }

    dotenvy::from_filename(format!(".{}.env", env)).ok();

    let port: u16 = convert_port_to_u16(&env::var("PORT").expect("PORT must be set")); 
    let app_host: String = env::var("APP_HOST").expect("APP_HOST must be set");
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    EnvFile { port, database_url, app_host}
}