use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod models;
mod controllers;
mod routes;
mod db;

use routes::user_routes::user_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // Initialize database connection pool
    db::init_pool(&database_url);

    HttpServer::new(|| {
        App::new().configure(user_routes)
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
