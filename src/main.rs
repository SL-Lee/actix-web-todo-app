use std::env;

use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{
    get, http::Cookie, post, web, App, HttpMessage, HttpRequest, HttpResponse,
    HttpServer, Responder,
};
use chrono::Utc;
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use dotenv::dotenv;
use rand::Rng;
use rand_core::OsRng;
use scrypt::{
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Scrypt,
};
use tera::{Context, Tera};

use actix_web_todo_app::{
    db::{
        models::{NewUser, User},
        schema::user,
    },
    forms::{LoginForm, SignupForm},
    scopes::api_scope,
    DbConnectionPool, Message,
};

#[get("/")]
async fn index(
    req: HttpRequest,
    identity: Identity,
    tera: web::Data<Tera>,
) -> impl Responder {
    if identity.identity().is_some() {
        return HttpResponse::Found().header("location", "/app").finish();
    }

    let mut context = Context::new();
    context.insert("is_logged_in", &identity.identity().is_some());
    context.insert("messages", &Vec::<Message>::new());

    if let Some(mut messages_cookie) = req.cookie("messages") {
        if let Ok(messages) =
            serde_json::from_str::<Vec<Message>>(messages_cookie.value())
        {
            context.insert("messages", &messages);
            messages_cookie.set_value("");
            return HttpResponse::Ok()
                .cookie(messages_cookie)
                .body(tera.render("index.html", &context).unwrap());
        }
    }

    HttpResponse::Ok().body(tera.render("index.html", &Context::new()).unwrap())
}

#[get("/login")]
async fn login(
    req: HttpRequest,
    identity: Identity,
    tera: web::Data<Tera>,
) -> impl Responder {
    if identity.identity().is_some() {
        return HttpResponse::Found().header("location", "/").finish();
    }

    let mut context = Context::new();
    context.insert("is_logged_in", &identity.identity().is_some());
    context.insert("messages", &Vec::<Message>::new());

    if let Some(mut messages_cookie) = req.cookie("messages") {
        if let Ok(messages) =
            serde_json::from_str::<Vec<Message>>(messages_cookie.value())
        {
            context.insert("messages", &messages);
            messages_cookie.set_value("");
            return HttpResponse::Ok()
                .cookie(messages_cookie)
                .body(tera.render("login.html", &context).unwrap());
        }
    }

    HttpResponse::Ok().body(tera.render("login.html", &Context::new()).unwrap())
}

#[post("/login")]
async fn process_login(
    req: HttpRequest,
    pool: DbConnectionPool,
    identity: Identity,
    form_data: web::Form<LoginForm>,
) -> impl Responder {
    if identity.identity().is_some() {
        return HttpResponse::Found().header("location", "/").finish();
    }

    let db_connection =
        pool.get().expect("Couldn't get db connection from pool");
    let user = user::table
        .filter(user::username.eq(&form_data.username))
        .first::<User>(&db_connection);

    match user {
        Ok(user) => {
            let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();

            match Scrypt
                .verify_password(form_data.password.as_bytes(), &parsed_hash)
            {
                Ok(_) => {
                    identity.remember(user.username.clone());
                    let mut messages_cookie = req.cookie("messages").unwrap_or(
                        Cookie::build("messages", "[]")
                            .same_site(actix_web::cookie::SameSite::Lax)
                            .finish(),
                    );
                    let mut messages = serde_json::from_str::<Vec<Message>>(
                        messages_cookie.value(),
                    )
                    .unwrap_or(vec![]);
                    messages.push(Message {
                        category: "success".to_string(),
                        title: "Login successful".to_string(),
                        content: "Logged in successfully.".to_string(),
                    });
                    messages_cookie
                        .set_value(serde_json::to_string(&messages).unwrap());
                    HttpResponse::Found()
                        .header("location", "/app")
                        .cookie(messages_cookie)
                        .finish()
                }
                Err(_) => {
                    let mut messages_cookie = req.cookie("messages").unwrap_or(
                        Cookie::build("messages", "[]")
                            .same_site(actix_web::cookie::SameSite::Lax)
                            .finish(),
                    );
                    let mut messages = serde_json::from_str::<Vec<Message>>(
                        messages_cookie.value(),
                    )
                    .unwrap_or(vec![]);
                    messages.push(Message {
                        category: "danger".to_string(),
                        title: "Login unsuccessful".to_string(),
                        content: "Incorrect username and/or password."
                            .to_string(),
                    });
                    messages_cookie
                        .set_value(serde_json::to_string(&messages).unwrap());
                    HttpResponse::Found()
                        .header("location", "/login")
                        .cookie(messages_cookie)
                        .finish()
                }
            }
        }
        Err(_) => {
            let mut messages_cookie = req.cookie("messages").unwrap_or(
                Cookie::build("messages", "[]")
                    .same_site(actix_web::cookie::SameSite::Lax)
                    .finish(),
            );
            let mut messages =
                serde_json::from_str::<Vec<Message>>(messages_cookie.value())
                    .unwrap_or(vec![]);
            messages.push(Message {
                category: "danger".to_string(),
                title: "Login unsuccessful".to_string(),
                content: "Incorrect username and/or password.".to_string(),
            });
            messages_cookie
                .set_value(serde_json::to_string(&messages).unwrap());
            HttpResponse::Found()
                .header("location", "/login")
                .cookie(messages_cookie)
                .finish()
        }
    }
}

#[get("/signup")]
async fn signup(
    req: HttpRequest,
    identity: Identity,
    tera: web::Data<Tera>,
) -> impl Responder {
    if identity.identity().is_some() {
        return HttpResponse::Found().header("location", "/").finish();
    }

    let mut context = Context::new();
    context.insert("is_logged_in", &identity.identity().is_some());
    context.insert("messages", &Vec::<Message>::new());

    if let Some(mut messages_cookie) = req.cookie("messages") {
        if let Ok(messages) =
            serde_json::from_str::<Vec<Message>>(messages_cookie.value())
        {
            context.insert("messages", &messages);
            messages_cookie.set_value("");
            return HttpResponse::Ok()
                .cookie(messages_cookie)
                .body(tera.render("signup.html", &context).unwrap());
        }
    }

    HttpResponse::Ok()
        .body(tera.render("signup.html", &Context::new()).unwrap())
}

#[post("/signup")]
async fn process_signup(
    req: HttpRequest,
    pool: DbConnectionPool,
    identity: Identity,
    form_data: web::Form<SignupForm>,
) -> impl Responder {
    if identity.identity().is_some() {
        return HttpResponse::Found().header("location", "/").finish();
    }

    let password = form_data.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Scrypt
        .hash_password_simple(password, salt.as_ref())
        .unwrap()
        .to_string();
    let new_user = NewUser {
        username: form_data.username.clone(),
        password_hash: password_hash.clone(),
        date_created: Some(Utc::now().naive_utc()),
    };
    let db_connection =
        pool.get().expect("Couldn't get db connection from pool");

    match web::block(move || {
        diesel::insert_into(user::table)
            .values(&new_user)
            .execute(&db_connection)
    })
    .await
    {
        Ok(_) => {
            identity.remember(form_data.username.clone());
            let mut messages_cookie = req.cookie("messages").unwrap_or(
                Cookie::build("messages", "[]")
                    .same_site(actix_web::cookie::SameSite::Lax)
                    .finish(),
            );
            let mut messages =
                serde_json::from_str::<Vec<Message>>(messages_cookie.value())
                    .unwrap_or(vec![]);
            messages.push(Message {
                category: "success".to_string(),
                title: "Signup successful".to_string(),
                content: "Signed up successfully.".to_string(),
            });
            messages_cookie
                .set_value(serde_json::to_string(&messages).unwrap());
            HttpResponse::Found()
                .header("location", "/app")
                .cookie(messages_cookie)
                .finish()
        }
        Err(_) => {
            let mut messages_cookie = req.cookie("messages").unwrap_or(
                Cookie::build("messages", "[]")
                    .same_site(actix_web::cookie::SameSite::Lax)
                    .finish(),
            );
            let mut messages =
                serde_json::from_str::<Vec<Message>>(messages_cookie.value())
                    .unwrap_or(vec![]);
            messages.push(Message {
                category: "danger".to_string(),
                title: "Signup unsuccessful".to_string(),
                content: "An account with this username already exists. \
                    Please try again with a different username."
                    .to_string(),
            });
            messages_cookie
                .set_value(serde_json::to_string(&messages).unwrap());
            HttpResponse::Found()
                .header("location", "/signup")
                .cookie(messages_cookie)
                .finish()
        }
    }
}

#[get("/logout")]
async fn logout(identity: Identity) -> impl Responder {
    if identity.identity().is_none() {
        return HttpResponse::NotFound().finish();
    }

    identity.forget();
    HttpResponse::Found().header("location", "/").finish()
}

#[get("/app")]
async fn app(
    req: HttpRequest,
    identity: Identity,
    tera: web::Data<Tera>,
) -> impl Responder {
    if identity.identity().is_none() {
        return HttpResponse::NotFound().finish();
    }

    let mut context = Context::new();
    context.insert("is_logged_in", &identity.identity().is_some());
    context.insert("messages", &Vec::<Message>::new());

    if let Some(mut messages_cookie) = req.cookie("messages") {
        if let Ok(messages) =
            serde_json::from_str::<Vec<Message>>(messages_cookie.value())
        {
            context.insert("messages", &messages);
            messages_cookie.set_value("");
            return HttpResponse::Ok()
                .cookie(messages_cookie)
                .body(tera.render("app.html", &context).unwrap());
        }
    }

    HttpResponse::Ok().body(tera.render("app.html", &context).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    dotenv().ok();
    let manager = ConnectionManager::<SqliteConnection>::new(
        env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    );
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

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
            .service(index)
            .service(login)
            .service(process_login)
            .service(signup)
            .service(process_signup)
            .service(logout)
            .service(app)
            .service(
                web::scope("/api")
                    .service(api_scope::create_todo)
                    .service(api_scope::get_todos)
                    .service(api_scope::update_todo)
                    .service(api_scope::update_todo_status)
                    .service(api_scope::delete_todo),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
