use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use actix_files as fs;
use actix_web::middleware::Logger;

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Academic group website";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(|| App::new()
        .wrap(Logger::default())
        .service(health_checker_handler)
        .service(index)
        .service(hello)
        .service(fs::Files::new("/static", "."))
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}