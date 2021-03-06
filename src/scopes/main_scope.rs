use actix_identity::Identity;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, Scope};
use chrono::Utc;
use diesel::prelude::*;
use rand_core::OsRng;
use scrypt::{
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Scrypt,
};
use tera::Tera;
use validator::Validate;

use crate::{
    create_message, create_response_for_template,
    db::{
        models::{NewUser, User},
        schema::user,
    },
    forms::{LoginForm, SignupForm},
    get_messages_cookie, initialize_context, DbConnectionPool,
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
async fn index(
    req: HttpRequest,
    identity: Identity,
    tera: web::Data<Tera>,
) -> impl Responder {
    if identity.identity().is_some() {
        return HttpResponse::Found().header("location", "/app").finish();
    }

    let mut context = initialize_context(&identity);
    create_response_for_template(&req, &mut context)
        .body(tera.render("index.html", &context).unwrap())
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

    let mut context = initialize_context(&identity);
    create_response_for_template(&req, &mut context)
        .body(tera.render("login.html", &context).unwrap())
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

    let mut messages_cookie = get_messages_cookie(&req);

    if let Err(validation_errors) = form_data.validate() {
        validation_errors.field_errors().iter().for_each(
            |(_, &field_errors)| {
                field_errors
                    .iter()
                    .filter_map(|error| error.message.as_ref())
                    .for_each(|error_message| {
                        create_message(
                            &mut messages_cookie,
                            "danger".to_string(),
                            "Login unsuccessful".to_string(),
                            error_message.to_string(),
                        );
                    });
            },
        );
        return HttpResponse::Found()
            .header("location", "/login")
            .cookie(messages_cookie)
            .finish();
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
                    create_message(
                        &mut messages_cookie,
                        "success".to_string(),
                        "Login successful".to_string(),
                        "Logged in successfully.".to_string(),
                    );
                    HttpResponse::Found()
                        .header("location", "/app")
                        .cookie(messages_cookie)
                        .finish()
                }
                Err(_) => {
                    create_message(
                        &mut messages_cookie,
                        "danger".to_string(),
                        "Login unsuccessful".to_string(),
                        "Incorrect username and/or password.".to_string(),
                    );
                    HttpResponse::Found()
                        .header("location", "/login")
                        .cookie(messages_cookie)
                        .finish()
                }
            }
        }
        Err(_) => {
            create_message(
                &mut messages_cookie,
                "danger".to_string(),
                "Login unsuccessful".to_string(),
                "Incorrect username and/or password.".to_string(),
            );
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

    let mut context = initialize_context(&identity);
    create_response_for_template(&req, &mut context)
        .body(tera.render("signup.html", &context).unwrap())
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

    let mut messages_cookie = get_messages_cookie(&req);

    if let Err(validation_errors) = form_data.validate() {
        validation_errors.field_errors().iter().for_each(
            |(_, &field_errors)| {
                field_errors
                    .iter()
                    .filter_map(|error| error.message.as_ref())
                    .for_each(|error_message| {
                        create_message(
                            &mut messages_cookie,
                            "danger".to_string(),
                            "Signup unsuccessful".to_string(),
                            error_message.to_string(),
                        );
                    });
            },
        );
        return HttpResponse::Found()
            .header("location", "/signup")
            .cookie(messages_cookie)
            .finish();
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
            create_message(
                &mut messages_cookie,
                "success".to_string(),
                "Signup successful".to_string(),
                "Signed up in successfully.".to_string(),
            );
            HttpResponse::Found()
                .header("location", "/app")
                .cookie(messages_cookie)
                .finish()
        }
        Err(_) => {
            create_message(
                &mut messages_cookie,
                "danger".to_string(),
                "Signup unsuccessful".to_string(),
                "An account with this username already exists. Please try \
                again with a different username."
                    .to_string(),
            );
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

    let mut context = initialize_context(&identity);
    create_response_for_template(&req, &mut context)
        .body(tera.render("app.html", &context).unwrap())
}
