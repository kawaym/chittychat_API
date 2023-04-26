use actix_web::{web, App, HttpServer};
use actix::SyncArbiter;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection
};

mod model;
mod routes;
mod utils;
mod database;
mod schema;

use crate::{database::setup::{get_pool, AppState, DbActor}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env: utils::load_envs::EnvFile = utils::load_envs::load();
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&env.database_url);   
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));
    println!("Server running on {}:{}", env.app_host, env.port);
    
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(AppState{db: db_addr.clone()}))
        .service(routes::index)
        .configure(routes::user::config)
    }).bind((env.app_host, env.port))?.run().await
}
