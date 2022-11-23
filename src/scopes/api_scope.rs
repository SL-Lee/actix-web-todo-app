use actix_identity::Identity;
use actix_web::web::{self, Form};
use actix_web::{delete, get, patch, post, put, HttpResponse, Responder, Scope};
use chrono::Utc;
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::db::models::{NewTodo, Todo};
use crate::db::schema::todo;
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
    identity: Option<Identity>,
    data: Form<CreateTodoEndpointData>,
) -> impl Responder {
    let user_id = match identity {
        Some(identity) => match Uuid::parse_str(&identity.id().unwrap()) {
            Ok(user_id) => user_id,
            Err(_) => return HttpResponse::BadRequest().finish(),
        },
        None => return HttpResponse::Unauthorized().finish(),
    };

    if let Err(validation_errors) = data.validate() {
        return HttpResponse::UnprocessableEntity()
            .json(json!({"status": "error", "fieldErrors": validation_errors.field_errors()}));
    }

    let mut db_connection = pool.get().expect("Couldn't get db connection from pool");
    let new_todo = NewTodo {
        title: data.todo_title.clone(),
        contents: data.todo_contents.clone(),
        completed: false,
        date_created: Utc::now().naive_utc(),
        user_id,
    };
    let create_todo_result = web::block(move || {
        diesel::insert_into(todo::table).values(&new_todo).get_result::<Todo>(&mut db_connection)
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
async fn get_todos(pool: DbConnectionPool, identity: Option<Identity>) -> impl Responder {
    let user_id = match identity {
        Some(identity) => match Uuid::parse_str(&identity.id().unwrap()) {
            Ok(user_id) => user_id,
            Err(_) => return HttpResponse::BadRequest().finish(),
        },
        None => return HttpResponse::Unauthorized().finish(),
    };
    let mut db_connection = pool.get().expect("Couldn't get db connection from pool");
    let todos = todo::table
        .filter(todo::user_id.eq(user_id))
        .order(todo::date_created)
        .get_results::<Todo>(&mut db_connection)
        .unwrap();
    HttpResponse::Ok().json(todos)
}

#[put("/todos")]
async fn update_todo(
    pool: DbConnectionPool,
    identity: Option<Identity>,
    data: Form<UpdateTodoEndpointData>,
) -> impl Responder {
    let user_id = match identity {
        Some(identity) => match Uuid::parse_str(&identity.id().unwrap()) {
            Ok(user_id) => user_id,
            Err(_) => return HttpResponse::BadRequest().finish(),
        },
        None => return HttpResponse::Unauthorized().finish(),
    };

    if let Err(validation_errors) = data.validate() {
        return HttpResponse::UnprocessableEntity()
            .json(json!({"status": "error", "fieldErrors": validation_errors.field_errors()}));
    }

    let mut db_connection = pool.get().expect("Couldn't get db connection from pool");
    let todo_user_id =
        match todo::table.filter(todo::id.eq(data.todo_id)).first::<Todo>(&mut db_connection) {
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
            .get_result::<Todo>(&mut db_connection)
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
    identity: Option<Identity>,
    data: Form<UpdateTodoStatusEndpointData>,
) -> impl Responder {
    let user_id = match identity {
        Some(identity) => match Uuid::parse_str(&identity.id().unwrap()) {
            Ok(user_id) => user_id,
            Err(_) => return HttpResponse::BadRequest().finish(),
        },
        None => return HttpResponse::Unauthorized().finish(),
    };

    let mut db_connection = pool.get().expect("Couldn't get db connection from pool");
    let todo_user_id =
        match todo::table.filter(todo::id.eq(data.todo_id)).first::<Todo>(&mut db_connection) {
            Ok(todo) => todo.user_id,
            Err(_) => return HttpResponse::Forbidden().finish(),
        };

    if user_id != todo_user_id {
        return HttpResponse::Forbidden().finish();
    }

    let update_todo_status_result = web::block(move || {
        diesel::update(todo::table.filter(todo::id.eq(data.todo_id).and(todo::user_id.eq(user_id))))
            .set(todo::completed.eq(data.todo_completed))
            .get_result::<Todo>(&mut db_connection)
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
    identity: Option<Identity>,
    data: Form<DeleteTodoEndpointData>,
) -> impl Responder {
    let user_id = match identity {
        Some(identity) => match Uuid::parse_str(&identity.id().unwrap()) {
            Ok(user_id) => user_id,
            Err(_) => return HttpResponse::BadRequest().finish(),
        },
        None => return HttpResponse::Unauthorized().finish(),
    };

    let mut db_connection = pool.get().expect("Couldn't get db connection from pool");
    let todo_user_id =
        match todo::table.filter(todo::id.eq(data.todo_id)).first::<Todo>(&mut db_connection) {
            Ok(todo) => todo.user_id,
            Err(_) => return HttpResponse::Forbidden().finish(),
        };

    if user_id != todo_user_id {
        return HttpResponse::Forbidden().finish();
    }

    let delete_todo_result = web::block(move || {
        diesel::delete(todo::table.filter(todo::id.eq(data.todo_id).and(todo::user_id.eq(user_id))))
            .get_result::<Todo>(&mut db_connection)
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
