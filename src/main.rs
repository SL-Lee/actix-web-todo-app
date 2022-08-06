use std::env;

use actix_identity::IdentityMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use tera::Tera;

use actix_web_todo_app::scopes::{api_scope, main_scope};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from the .env file
    dotenv().ok();

    // Initializes the global logger with an env logger
    env_logger::init();

    // Generate the private key used for the identity service
    let private_key = actix_web::cookie::Key::generate();

    // Create database connection pool
    let manager = ConnectionManager::<PgConnection>::new(
        env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    );
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(r#"%a "%r" %s [%Ts]"#))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), private_key.clone()))
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(Tera::new("templates/**/*").unwrap()))
            .service(actix_files::Files::new("/static", "./static"))
            .service(api_scope::get_scope())
            .service(main_scope::get_scope())
    })
    .bind(env::var("SERVER_URL").unwrap_or_else(|_| "127.0.0.1:8080".to_string()))?
    .run()
    .await
}
