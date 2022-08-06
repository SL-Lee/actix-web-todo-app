#[macro_use]
extern crate diesel;

pub mod db;
pub mod forms;
pub mod scopes;

use actix_identity::Identity;
use actix_web::cookie::Cookie;
use actix_web::HttpResponseBuilder;
use actix_web::{web, HttpRequest, HttpResponse};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tera::Context;

pub type DbConnectionPool = web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>;

#[derive(Debug, Deserialize, Serialize)]
struct Message<'a> {
    category: &'a str,
    title: &'a str,
    content: &'a str,
}

trait CreateMessage {
    fn create_message(&mut self, category: &str, title: &str, content: &str);
}

impl CreateMessage for Cookie<'_> {
    fn create_message(&mut self, category: &str, title: &str, content: &str) {
        let mut messages = serde_json::from_str::<Vec<Message>>(self.value()).unwrap_or_default();
        messages.push(Message { category, title, content });
        self.set_value(serde_json::to_string(&messages).unwrap());
    }
}

pub fn initialize_context(identity: Option<Identity>) -> Context {
    Context::from_value(json!({
        "is_logged_in": identity.is_some(),
        "messages": &Vec::<Message>::new()
    }))
    .unwrap()
}

pub fn create_response_for_template(
    req: &HttpRequest,
    context: &mut Context,
) -> HttpResponseBuilder {
    match req.cookie("messages") {
        Some(mut messages_cookie) => {
            // For some reason, retrieving the cookie using `req.cookie` will unset the retrieved
            // cookie's `SameSite` attribute to `None`. To preserve the SameSite attribute's value
            // of `Lax`, we will have to re-set the cookie's SameSite attribute every time we
            // retrieve it (and intend to write it back to the client).
            messages_cookie.set_same_site(actix_web::cookie::SameSite::Lax);

            match serde_json::from_str::<Vec<Message>>(messages_cookie.value()) {
                Ok(messages) => {
                    context.insert("messages", &messages);
                    messages_cookie.set_value("[]");
                    HttpResponse::Ok().cookie(messages_cookie).take()
                }
                Err(_) => HttpResponse::Ok(),
            }
        }
        None => HttpResponse::Ok(),
    }
}

pub fn get_messages_cookie(req: &HttpRequest) -> Cookie {
    req.cookie("messages").map_or(
        Cookie::build("messages", "[]").same_site(actix_web::cookie::SameSite::Lax).finish(),
        |mut messages_cookie| {
            // For some reason, retrieving the cookie using `req.cookie` will unset the retrieved
            // cookie's `SameSite` attribute to `None`. To preserve the SameSite attribute's value
            // of `Lax`, we will have to re-set the cookie's SameSite attribute every time we
            // retrieve it (and intend to write it back to the client).
            messages_cookie.set_same_site(actix_web::cookie::SameSite::Lax);
            messages_cookie
        },
    )
}
