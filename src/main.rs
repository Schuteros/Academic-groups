mod config;
mod handler;
mod jwt_auth;
mod model;
mod response;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer, Result, Error};
use config::Config;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use actix_files as fs;
use actix_files::NamedFile;

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

async fn login_form() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("static/login.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                env: config.clone(),
            }))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
            .route("/auth/login", web::get().to(login_form))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
