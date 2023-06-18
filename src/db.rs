use mysql::Pool;
use once_cell::sync::Lazy;
use std::env;

pub static POOL: Lazy<Pool> = Lazy::new(|| {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    Pool::new(database_url).expect("Failed to create database pool")
});

pub fn establish_connection() -> Pool {
    POOL.clone()
}

pub fn init_pool(database_url: &str) {
    let _ = dotenv::from_filename(".env");
    let _ = env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL not set"));
    POOL.clone();
}
