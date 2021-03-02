#[macro_use]
extern crate diesel;

pub mod db;
pub mod forms;
pub mod scopes;

use actix_web::web;
use diesel::{
    r2d2::{self, ConnectionManager},
    sqlite::SqliteConnection,
};
use serde::{Deserialize, Serialize};

pub type DbConnectionPool =
    web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub category: String,
    pub title: String,
    pub content: String,
}
