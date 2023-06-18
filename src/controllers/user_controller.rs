use actix_web::{HttpResponse};
use mysql::prelude::*;
use crate::models::user::User;
use crate::db::establish_connection;

pub async fn get_users() -> HttpResponse {
    let pool = establish_connection();
    let mut conn = pool.get_conn().unwrap();

    let users: Vec<User> = conn.query_map(
        "SELECT id, name, email FROM users",
        |(id, name, email)| User { id, name, email },
    ).unwrap();

    HttpResponse::Ok().json(users)
}
