use actix_web::{
    get, http, post, web::{Json, Path}, Error, HttpResponse, Responder
};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct NewTodo {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[utoipa::path(summary = "Get an element from the todo list", responses((status = OK, body = Todo)),params(("todoId", description = "Todo id"),))]
#[get("/{todoId}")]
pub async fn getTodo(todoId: Path<String>) -> Result<impl Responder, Error> {
    Ok(Json(Todo {
        id: todoId.into_inner(),
        title: "some title".to_string(),
        description: None,
    }))
}

#[utoipa::path(summary = "Add a new element to the todo list", responses((status = CREATED, body = Todo)))]
#[post("/")]
pub async fn addTodo(body: Json<NewTodo>) -> Result<impl Responder, Error> {
    let newTodo = body.into_inner();
    Ok(HttpResponse::Created().json(Todo {
        id: nanoid!(),
        title: newTodo.title,
        description: newTodo.description,
    }))
}
