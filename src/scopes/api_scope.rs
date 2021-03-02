use actix_identity::Identity;
use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder};
use diesel::prelude::*;
use json::object;

use crate::{
    db::{
        models::{NewTodo, Todo, User},
        schema::{todo, user},
    },
    forms::{
        CreateTodoEndpointData, DeleteTodoEndpointData, UpdateTodoEndpointData,
        UpdateTodoStatusEndpointData,
    },
    DbConnectionPool,
};

#[post("/todos")]
pub async fn create_todo(
    pool: DbConnectionPool,
    identity: Identity,
    data: web::Form<CreateTodoEndpointData>,
) -> impl Responder {
    if identity.identity().is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let db_connection =
        pool.get().expect("Couldn't get db connection from pool");
    let user_id = user::table
        .filter(user::username.eq(&identity.identity().unwrap()))
        .first::<User>(&db_connection)
        .unwrap()
        .id;
    let new_todo = NewTodo {
        title: data.todo_title.clone(),
        contents: data.todo_contents.clone(),
        completed: false,
        user_id: Some(user_id),
    };
    web::block(move || {
        diesel::insert_into(todo::table)
            .values(&new_todo)
            .execute(&db_connection)
    })
    .await
    .unwrap();
    HttpResponse::Created()
        .content_type("application/json")
        .body(object! {status: "Success"}.dump())
}

#[get("/todos")]
pub async fn get_todos(
    pool: DbConnectionPool,
    identity: Identity,
) -> impl Responder {
    if identity.identity().is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let db_connection =
        pool.get().expect("Couldn't get db connection from pool");
    let user_id = user::table
        .filter(user::username.eq(&identity.identity().unwrap()))
        .first::<User>(&db_connection)
        .unwrap()
        .id;
    let todos = todo::table
        .filter(todo::user_id.eq(Some(user_id)))
        .get_results::<Todo>(&db_connection)
        .unwrap();
    HttpResponse::Ok().json(todos)
}

#[put("/todos")]
pub async fn update_todo(
    pool: DbConnectionPool,
    identity: Identity,
    data: web::Form<UpdateTodoEndpointData>,
) -> impl Responder {
    if identity.identity().is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let db_connection =
        pool.get().expect("Couldn't get db connection from pool");
    web::block(move || {
        diesel::update(todo::table.filter(todo::id.eq(data.todo_id)))
            .set((
                todo::title.eq(data.todo_title.clone()),
                todo::contents.eq(data.todo_contents.clone()),
            ))
            .execute(&db_connection)
    })
    .await
    .unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(object! {status: "Success"}.dump())
}

#[patch("/todos")]
pub async fn update_todo_status(
    pool: DbConnectionPool,
    identity: Identity,
    data: web::Form<UpdateTodoStatusEndpointData>,
) -> impl Responder {
    if identity.identity().is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let db_connection =
        pool.get().expect("Couldn't get db connection from pool");
    web::block(move || {
        diesel::update(todo::table.filter(todo::id.eq(data.todo_id)))
            .set(todo::completed.eq(data.todo_completed))
            .execute(&db_connection)
    })
    .await
    .unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(object! {status: "Success"}.dump())
}

#[delete("/todos")]
pub async fn delete_todo(
    pool: DbConnectionPool,
    identity: Identity,
    data: web::Form<DeleteTodoEndpointData>,
) -> impl Responder {
    if identity.identity().is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let db_connection =
        pool.get().expect("Couldn't get db connection from pool");
    web::block(move || {
        diesel::delete(todo::table.filter(todo::id.eq(data.todo_id)))
            .execute(&db_connection)
    })
    .await
    .unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(object! {status: "Success"}.dump())
}
