use actix_identity::Identity;
use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder, Scope};
use diesel::prelude::*;
use serde_json::json;

use crate::db::models::{NewTodo, Todo, User};
use crate::db::schema::{todo, user};
use crate::forms::{
    CreateTodoEndpointData, DeleteTodoEndpointData, UpdateTodoEndpointData,
    UpdateTodoStatusEndpointData,
};
use crate::DbConnectionPool;

pub fn get_scope() -> Scope {
    web::scope("/api")
        .service(create_todo)
        .service(get_todos)
        .service(update_todo)
        .service(update_todo_status)
        .service(delete_todo)
}

#[post("/todos")]
async fn create_todo(
    pool: DbConnectionPool,
    identity: Identity,
    data: web::Form<CreateTodoEndpointData>,
) -> impl Responder {
    if identity.identity().is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let db_connection = pool.get().expect("Couldn't get db connection from pool");
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
    web::block(move || diesel::insert_into(todo::table).values(&new_todo).execute(&db_connection))
        .await
        .unwrap();
    HttpResponse::Created().json(json!({"status": "Success"}))
}

#[get("/todos")]
async fn get_todos(pool: DbConnectionPool, identity: Identity) -> impl Responder {
    if identity.identity().is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let db_connection = pool.get().expect("Couldn't get db connection from pool");
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
async fn update_todo(
    pool: DbConnectionPool,
    identity: Identity,
    data: web::Form<UpdateTodoEndpointData>,
) -> impl Responder {
    if identity.identity().is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let db_connection = pool.get().expect("Couldn't get db connection from pool");
    let user_id = user::table
        .filter(user::username.eq(&identity.identity().unwrap()))
        .first::<User>(&db_connection)
        .unwrap()
        .id;
    let todo_user_id =
        match todo::table.filter(todo::id.eq(data.todo_id)).first::<Todo>(&db_connection) {
            Ok(todo) => todo.user_id.unwrap(),
            Err(_) => return HttpResponse::Forbidden().finish(),
        };

    if user_id != todo_user_id {
        return HttpResponse::Forbidden().finish();
    }

    web::block(move || {
        diesel::update(
            todo::table.filter(todo::id.eq(data.todo_id).and(todo::user_id.eq(Some(user_id)))),
        )
        .set((
            todo::title.eq(data.todo_title.clone()),
            todo::contents.eq(data.todo_contents.clone()),
        ))
        .execute(&db_connection)
    })
    .await
    .unwrap();
    HttpResponse::Ok().json(json!({"status": "Success"}))
}

#[patch("/todos")]
async fn update_todo_status(
    pool: DbConnectionPool,
    identity: Identity,
    data: web::Form<UpdateTodoStatusEndpointData>,
) -> impl Responder {
    if identity.identity().is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let db_connection = pool.get().expect("Couldn't get db connection from pool");
    let user_id = user::table
        .filter(user::username.eq(&identity.identity().unwrap()))
        .first::<User>(&db_connection)
        .unwrap()
        .id;
    let todo_user_id =
        match todo::table.filter(todo::id.eq(data.todo_id)).first::<Todo>(&db_connection) {
            Ok(todo) => todo.user_id.unwrap(),
            Err(_) => return HttpResponse::Forbidden().finish(),
        };

    if user_id != todo_user_id {
        return HttpResponse::Forbidden().finish();
    }

    web::block(move || {
        diesel::update(
            todo::table.filter(todo::id.eq(data.todo_id).and(todo::user_id.eq(Some(user_id)))),
        )
        .set(todo::completed.eq(data.todo_completed))
        .execute(&db_connection)
    })
    .await
    .unwrap();
    HttpResponse::Ok().json(json!({"status": "Success"}))
}

#[delete("/todos")]
async fn delete_todo(
    pool: DbConnectionPool,
    identity: Identity,
    data: web::Form<DeleteTodoEndpointData>,
) -> impl Responder {
    if identity.identity().is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let db_connection = pool.get().expect("Couldn't get db connection from pool");
    let user_id = user::table
        .filter(user::username.eq(&identity.identity().unwrap()))
        .first::<User>(&db_connection)
        .unwrap()
        .id;
    let todo_user_id =
        match todo::table.filter(todo::id.eq(data.todo_id)).first::<Todo>(&db_connection) {
            Ok(todo) => todo.user_id.unwrap(),
            Err(_) => return HttpResponse::Forbidden().finish(),
        };

    if user_id != todo_user_id {
        return HttpResponse::Forbidden().finish();
    }

    web::block(move || {
        diesel::delete(
            todo::table.filter(todo::id.eq(data.todo_id).and(todo::user_id.eq(Some(user_id)))),
        )
        .execute(&db_connection)
    })
    .await
    .unwrap();
    HttpResponse::Ok().json(json!({"status": "Success"}))
}
