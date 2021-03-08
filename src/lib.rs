#[macro_use]
extern crate diesel;

pub mod db;
pub mod forms;
pub mod scopes;

use actix_identity::Identity;
use actix_web::{
    dev::HttpResponseBuilder, http::Cookie, web, HttpMessage, HttpRequest,
    HttpResponse,
};
use diesel::{
    r2d2::{self, ConnectionManager},
    sqlite::SqliteConnection,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tera::Context;

pub type DbConnectionPool =
    web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub category: String,
    pub title: String,
    pub content: String,
}

pub fn initialize_context(identity: &Identity) -> Context {
    Context::from_value(json!({
        "is_logged_in": identity.identity().is_some(),
        "messages": &Vec::<Message>::new()
    }))
    .unwrap()
}

pub fn create_response_for_template(
    req: &HttpRequest,
    context: &mut Context,
) -> HttpResponseBuilder {
    if let Some(mut messages_cookie) = req.cookie("messages") {
        if let Ok(messages) =
            serde_json::from_str::<Vec<Message>>(messages_cookie.value())
        {
            context.insert("messages", &messages);
            messages_cookie.set_value("[]");
            HttpResponse::Ok().cookie(messages_cookie).take()
        } else {
            HttpResponse::Ok()
        }
    } else {
        HttpResponse::Ok()
    }
}

pub fn get_messages_cookie(req: &HttpRequest) -> Cookie {
    req.cookie("messages").unwrap_or(
        Cookie::build("messages", "[]")
            .same_site(actix_web::cookie::SameSite::Lax)
            .finish(),
    )
}

pub fn create_message(
    messages_cookie: &mut Cookie,
    message_category: String,
    message_title: String,
    message_content: String,
) {
    let mut messages =
        serde_json::from_str::<Vec<Message>>(messages_cookie.value())
            .unwrap_or(vec![]);
    messages.push(Message {
        category: message_category,
        title: message_title,
        content: message_content,
    });
    messages_cookie.set_value(serde_json::to_string(&messages).unwrap());
}
