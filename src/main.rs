use std::env;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use rand::Rng;
use tera::Tera;

use actix_web_todo_app::scopes::{api_scope, main_scope};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    dotenv().ok();
    let manager = ConnectionManager::<SqliteConnection>::new(
        env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    );
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .name("session")
                    .same_site(actix_web::cookie::SameSite::Lax)
                    .secure(false),
            ))
            .data(pool.clone())
            .data(Tera::new("templates/**/*").unwrap())
            .service(actix_files::Files::new("/static", "./static"))
            .service(api_scope::get_scope())
            .service(main_scope::get_scope())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
