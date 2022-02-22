use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::db::schema::{todo, user};

#[derive(Queryable, Serialize)]
pub struct Todo {
    pub id: Uuid,

    pub title: String,

    pub contents: Option<String>,

    pub completed: bool,

    pub date_created: NaiveDateTime,

    #[serde(skip_serializing)]
    pub user_id: Uuid,
}

#[derive(Insertable)]
#[table_name = "todo"]
pub struct NewTodo {
    pub title: String,
    pub contents: Option<String>,
    pub completed: bool,
    pub date_created: NaiveDateTime,
    pub user_id: Uuid,
}

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub date_created: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
    pub date_created: NaiveDateTime,
}
