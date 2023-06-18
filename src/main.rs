use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/api/resource")]
async fn create_resource(_payload: web::Json<CreateResourceRequest>) -> impl Responder {
    // Handle the POST request and return an appropriate response
    HttpResponse::Ok().body("Resource created successfully")
}

#[derive(serde::Deserialize)]
struct CreateResourceRequest {
    // Define the request payload structure here
    // Example: `name: String, age: u32, etc.`
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(create_resource)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
