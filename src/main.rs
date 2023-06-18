use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod models;
mod controllers;
mod routes;
mod db;

use crate::controllers::user_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // Initialize database connection pool
    db::init_pool(&database_url);

    HttpServer::new(|| {
        App::new().configure(user_controller::get_users)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
