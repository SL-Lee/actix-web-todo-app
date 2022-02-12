use chrono::NaiveDateTime;
use serde::Serialize;

use crate::db::schema::{todo, user};

#[derive(Queryable, Serialize)]
pub struct Todo {
    pub id: i32,

    pub title: String,

    pub contents: Option<String>,

    pub completed: bool,

    #[serde(skip_serializing)]
    pub user_id: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "todo"]
pub struct NewTodo {
    pub title: String,
    pub contents: Option<String>,
    pub completed: bool,
    pub user_id: Option<i32>,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub date_created: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
    pub date_created: Option<NaiveDateTime>,
}
