use actix_identity::Identity;
use actix_web::web::{self, Data, Form};
use actix_web::{get, post, HttpMessage, HttpRequest, HttpResponse, Responder, Scope};
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use chrono::Utc;
use diesel::prelude::*;
use rand_core::OsRng;
use tera::Tera;
use validator::Validate;

use crate::db::models::{NewUser, User};
use crate::db::schema::user;
use crate::forms::{LoginForm, SignupForm};
use crate::{
    create_response_for_template, get_messages_cookie, initialize_context, CreateMessage,
    DbConnectionPool,
};

pub fn get_scope() -> Scope {
    web::scope("")
        .service(index)
        .service(login)
        .service(process_login)
        .service(signup)
        .service(process_signup)
        .service(logout)
        .service(app)
}

#[get("/")]
async fn index(req: HttpRequest, identity: Option<Identity>, tera: Data<Tera>) -> impl Responder {
    if identity.is_some() {
        return HttpResponse::Found().append_header(("location", "/app")).finish();
    }

    let mut context = initialize_context(identity);
    create_response_for_template(&req, &mut context)
        .body(tera.render("index.html", &context).unwrap())
}

#[get("/login")]
async fn login(req: HttpRequest, identity: Option<Identity>, tera: Data<Tera>) -> impl Responder {
    if identity.is_some() {
        return HttpResponse::Found().append_header(("location", "/")).finish();
    }

    let mut context = initialize_context(identity);
    create_response_for_template(&req, &mut context)
        .body(tera.render("login.html", &context).unwrap())
}

#[post("/login")]
async fn process_login(
    req: HttpRequest,
    pool: DbConnectionPool,
    identity: Option<Identity>,
    form_data: Form<LoginForm>,
) -> impl Responder {
    if identity.is_some() {
        return HttpResponse::Found().append_header(("location", "/")).finish();
    }

    let mut messages_cookie = get_messages_cookie(&req);

    if let Err(validation_errors) = form_data.validate() {
        validation_errors.field_errors().iter().for_each(|(_, &field_errors)| {
            field_errors.iter().filter_map(|error| error.message.as_ref()).for_each(
                |error_message| {
                    messages_cookie.create_message("danger", "Login unsuccessful", error_message);
                },
            );
        });
        return HttpResponse::Found()
            .append_header(("location", "/login"))
            .cookie(messages_cookie)
            .finish();
    }

    let db_connection = pool.get().expect("Couldn't get db connection from pool");
    let get_user_result =
        user::table.filter(user::username.eq(&form_data.username)).first::<User>(&db_connection);

    match get_user_result {
        Ok(user) => {
            if Argon2::default()
                .verify_password(
                    form_data.password.as_bytes(),
                    &PasswordHash::new(&user.password_hash).unwrap(),
                )
                .is_ok()
            {
                let _ = Identity::login(&req.extensions(), user.id.to_string());
                messages_cookie.create_message(
                    "success",
                    "Login successful",
                    "Logged in successfully.",
                );
                HttpResponse::Found()
                    .append_header(("location", "/app"))
                    .cookie(messages_cookie)
                    .finish()
            } else {
                messages_cookie.create_message(
                    "danger",
                    "Login unsuccessful",
                    "Incorrect username and/or password.",
                );
                HttpResponse::Found()
                    .append_header(("location", "/login"))
                    .cookie(messages_cookie)
                    .finish()
            }
        }
        Err(_) => {
            messages_cookie.create_message(
                "danger",
                "Login unsuccessful",
                "Incorrect username and/or password.",
            );
            HttpResponse::Found()
                .append_header(("location", "/login"))
                .cookie(messages_cookie)
                .finish()
        }
    }
}

#[get("/signup")]
async fn signup(req: HttpRequest, identity: Option<Identity>, tera: Data<Tera>) -> impl Responder {
    if identity.is_some() {
        return HttpResponse::Found().append_header(("location", "/")).finish();
    }

    let mut context = initialize_context(identity);
    create_response_for_template(&req, &mut context)
        .body(tera.render("signup.html", &context).unwrap())
}

#[post("/signup")]
async fn process_signup(
    req: HttpRequest,
    pool: DbConnectionPool,
    identity: Option<Identity>,
    form_data: Form<SignupForm>,
) -> impl Responder {
    if identity.is_some() {
        return HttpResponse::Found().append_header(("location", "/")).finish();
    }

    let mut messages_cookie = get_messages_cookie(&req);

    if let Err(validation_errors) = form_data.validate() {
        validation_errors.field_errors().iter().for_each(|(_, &field_errors)| {
            field_errors.iter().filter_map(|error| error.message.as_ref()).for_each(
                |error_message| {
                    messages_cookie.create_message("danger", "Signup unsuccessful", error_message);
                },
            );
        });
        return HttpResponse::Found()
            .append_header(("location", "/signup"))
            .cookie(messages_cookie)
            .finish();
    }

    let password = form_data.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default().hash_password(password, &salt).unwrap().to_string();
    let new_user = NewUser {
        username: form_data.username.clone(),
        password_hash: password_hash.clone(),
        date_created: Utc::now().naive_utc(),
    };
    let db_connection = pool.get().expect("Couldn't get db connection from pool");

    match web::block(move || {
        diesel::insert_into(user::table).values(&new_user).get_result::<User>(&db_connection)
    })
    .await
    {
        Ok(Ok(new_user)) => {
            let _ = Identity::login(&req.extensions(), new_user.id.to_string());
            messages_cookie.create_message(
                "success",
                "Signup successful",
                "Signed up successfully.",
            );
            HttpResponse::Found()
                .append_header(("location", "/app"))
                .cookie(messages_cookie)
                .finish()
        }
        _ => {
            messages_cookie.create_message(
                "danger",
                "Signup unsuccessful",
                "An account with this username already exists. Please try again with a different \
                username.",
            );
            HttpResponse::Found()
                .append_header(("location", "/signup"))
                .cookie(messages_cookie)
                .finish()
        }
    }
}

#[get("/logout")]
async fn logout(identity: Option<Identity>) -> impl Responder {
    match identity {
        Some(identity) => {
            identity.logout();
            HttpResponse::Found().append_header(("location", "/")).finish()
        }
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/app")]
async fn app(req: HttpRequest, identity: Option<Identity>, tera: Data<Tera>) -> impl Responder {
    if identity.is_none() {
        return HttpResponse::NotFound().finish();
    }

    let mut context = initialize_context(identity);
    create_response_for_template(&req, &mut context)
        .body(tera.render("app.html", &context).unwrap())
}
