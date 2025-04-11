use actix_web::body::BoxBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::from_fn;
use actix_web::web::{Form, ReqData, block, scope};
use actix_web::{HttpResponse, Responder, Scope, delete, get, patch, post, put};
use chrono::Utc;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{delete, insert_into, update};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::DbConnectionPool;
use crate::db::models::{NewTodo, Todo};
use crate::db::schema::todo;
use crate::forms::{
    CreateTodoEndpointData, DeleteTodoEndpointData, UpdateTodoEndpointData,
    UpdateTodoStatusEndpointData,
};
use crate::middleware::user_id_middleware;

pub fn get_scope() -> Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<BoxBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    scope("/api")
        .wrap(from_fn(user_id_middleware))
        .service(create_todo)
        .service(get_todos)
        .service(update_todo)
        .service(update_todo_status)
        .service(delete_todo)
}

#[post("/todos")]
async fn create_todo(
    pool: DbConnectionPool,
    user_id: ReqData<Uuid>,
    data: Form<CreateTodoEndpointData>,
) -> impl Responder {
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
        user_id: user_id.into_inner(),
    };
    let create_todo_fut = block(move || {
        insert_into(todo::table).values(&new_todo).get_result::<Todo>(&mut db_connection)
    });
    match create_todo_fut.await {
        Ok(Ok(todo)) => HttpResponse::Created().json(json!({"status": "success", "todo": todo})),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/todos")]
async fn get_todos(pool: DbConnectionPool, user_id: ReqData<Uuid>) -> impl Responder {
    let mut db_connection = pool.get().expect("Couldn't get db connection from pool");
    let todos = todo::table
        .filter(todo::user_id.eq(user_id.into_inner()))
        .order(todo::date_created)
        .get_results::<Todo>(&mut db_connection)
        .unwrap();
    HttpResponse::Ok().json(todos)
}

#[put("/todos")]
async fn update_todo(
    pool: DbConnectionPool,
    user_id: ReqData<Uuid>,
    data: Form<UpdateTodoEndpointData>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    if let Err(validation_errors) = data.validate() {
        return HttpResponse::UnprocessableEntity()
            .json(json!({"status": "error", "fieldErrors": validation_errors.field_errors()}));
    }

    let mut db_connection = pool.get().expect("Couldn't get db connection from pool");
    if !is_todo_owned_by_user(data.todo_id, user_id, &mut db_connection) {
        return HttpResponse::Forbidden().finish();
    }

    let update_todo_fut = block(move || {
        update(todo::table.filter(todo::id.eq(data.todo_id).and(todo::user_id.eq(user_id))))
            .set((
                todo::title.eq(data.todo_title.clone()),
                todo::contents.eq(data.todo_contents.clone()),
            ))
            .get_result::<Todo>(&mut db_connection)
    });
    match update_todo_fut.await {
        Ok(Ok(todo)) => HttpResponse::Ok().json(json!({"status": "success", "todo": todo})),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[patch("/todos")]
async fn update_todo_status(
    pool: DbConnectionPool,
    user_id: ReqData<Uuid>,
    data: Form<UpdateTodoStatusEndpointData>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    let mut db_connection = pool.get().expect("Couldn't get db connection from pool");
    if !is_todo_owned_by_user(data.todo_id, user_id, &mut db_connection) {
        return HttpResponse::Forbidden().finish();
    }

    let update_todo_status_fut = block(move || {
        update(todo::table.filter(todo::id.eq(data.todo_id).and(todo::user_id.eq(user_id))))
            .set(todo::completed.eq(data.todo_completed))
            .get_result::<Todo>(&mut db_connection)
    });
    match update_todo_status_fut.await {
        Ok(Ok(todo)) => {
            HttpResponse::Ok().json(json!({"status": "success", "newTodoStatus": todo.completed}))
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/todos")]
async fn delete_todo(
    pool: DbConnectionPool,
    user_id: ReqData<Uuid>,
    data: Form<DeleteTodoEndpointData>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    let mut db_connection = pool.get().expect("Couldn't get db connection from pool");
    if !is_todo_owned_by_user(data.todo_id, user_id, &mut db_connection) {
        return HttpResponse::Forbidden().finish();
    }

    let delete_todo_fut = block(move || {
        delete(todo::table.filter(todo::id.eq(data.todo_id).and(todo::user_id.eq(user_id))))
            .get_result::<Todo>(&mut db_connection)
    });
    match delete_todo_fut.await {
        Ok(Ok(todo)) => {
            HttpResponse::Ok().json(json!({"status": "success", "deletedTodoId": todo.id}))
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

fn is_todo_owned_by_user(
    todo_id: Uuid,
    user_id: Uuid,
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> bool {
    let query_result = todo::table.filter(todo::id.eq(todo_id)).first::<Todo>(db_connection);
    query_result.is_ok() && query_result.unwrap().user_id == user_id
}
