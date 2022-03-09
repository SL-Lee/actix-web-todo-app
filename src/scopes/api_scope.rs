use actix_identity::Identity;
use actix_web::web::{self, Form};
use actix_web::{delete, get, patch, post, put, HttpResponse, Responder, Scope};
use chrono::Utc;
use diesel::prelude::*;
use serde_json::json;
use validator::Validate;

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
    data: Form<CreateTodoEndpointData>,
) -> impl Responder {
    if identity.identity().is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    if let Err(validation_errors) = data.validate() {
        return HttpResponse::UnprocessableEntity()
            .json(json!({"status": "error", "fieldErrors": validation_errors.field_errors()}));
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
        date_created: Utc::now().naive_utc(),
        user_id,
    };

    let create_todo_result = web::block(move || {
        diesel::insert_into(todo::table).values(&new_todo).get_result::<Todo>(&db_connection)
    })
    .await;

    // There are two layers of results here -- the outer Result is the result of executing the
    // blocking function, while the inner result is the Result returned by the blocking function.
    match create_todo_result {
        Ok(Ok(todo)) => HttpResponse::Created().json(json!({"status": "success", "todo": todo})),
        _ => HttpResponse::InternalServerError().finish(),
    }
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
        .filter(todo::user_id.eq(user_id))
        .order(todo::date_created)
        .get_results::<Todo>(&db_connection)
        .unwrap();
    HttpResponse::Ok().json(todos)
}

#[put("/todos")]
async fn update_todo(
    pool: DbConnectionPool,
    identity: Identity,
    data: Form<UpdateTodoEndpointData>,
) -> impl Responder {
    if identity.identity().is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    if let Err(validation_errors) = data.validate() {
        return HttpResponse::UnprocessableEntity()
            .json(json!({"status": "error", "fieldErrors": validation_errors.field_errors()}));
    }

    let db_connection = pool.get().expect("Couldn't get db connection from pool");
    let user_id = user::table
        .filter(user::username.eq(&identity.identity().unwrap()))
        .first::<User>(&db_connection)
        .unwrap()
        .id;
    let todo_user_id =
        match todo::table.filter(todo::id.eq(data.todo_id)).first::<Todo>(&db_connection) {
            Ok(todo) => todo.user_id,
            Err(_) => return HttpResponse::Forbidden().finish(),
        };

    if user_id != todo_user_id {
        return HttpResponse::Forbidden().finish();
    }

    let update_todo_result = web::block(move || {
        diesel::update(todo::table.filter(todo::id.eq(data.todo_id).and(todo::user_id.eq(user_id))))
            .set((
                todo::title.eq(data.todo_title.clone()),
                todo::contents.eq(data.todo_contents.clone()),
            ))
            .get_result::<Todo>(&db_connection)
    })
    .await;

    // There are two layers of results here -- the outer Result is the result of executing the
    // blocking function, while the inner result is the Result returned by the blocking function.
    match update_todo_result {
        Ok(Ok(todo)) => HttpResponse::Ok().json(json!({"status": "success", "todo": todo})),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[patch("/todos")]
async fn update_todo_status(
    pool: DbConnectionPool,
    identity: Identity,
    data: Form<UpdateTodoStatusEndpointData>,
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
            Ok(todo) => todo.user_id,
            Err(_) => return HttpResponse::Forbidden().finish(),
        };

    if user_id != todo_user_id {
        return HttpResponse::Forbidden().finish();
    }

    let update_todo_status_result = web::block(move || {
        diesel::update(todo::table.filter(todo::id.eq(data.todo_id).and(todo::user_id.eq(user_id))))
            .set(todo::completed.eq(data.todo_completed))
            .get_result::<Todo>(&db_connection)
    })
    .await;

    // There are two layers of results here -- the outer Result is the result of executing the
    // blocking function, while the inner result is the Result returned by the blocking function.
    match update_todo_status_result {
        Ok(Ok(todo)) => {
            HttpResponse::Ok().json(json!({"status": "success", "newTodoStatus": todo.completed}))
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/todos")]
async fn delete_todo(
    pool: DbConnectionPool,
    identity: Identity,
    data: Form<DeleteTodoEndpointData>,
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
            Ok(todo) => todo.user_id,
            Err(_) => return HttpResponse::Forbidden().finish(),
        };

    if user_id != todo_user_id {
        return HttpResponse::Forbidden().finish();
    }

    let delete_todo_result = web::block(move || {
        diesel::delete(todo::table.filter(todo::id.eq(data.todo_id).and(todo::user_id.eq(user_id))))
            .get_result::<Todo>(&db_connection)
    })
    .await;

    // There are two layers of results here -- the outer Result is the result of executing the
    // blocking function, while the inner result is the Result returned by the blocking function.
    match delete_todo_result {
        Ok(Ok(todo)) => {
            HttpResponse::Ok().json(json!({"status": "success", "deletedTodoId": todo.id}))
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}
