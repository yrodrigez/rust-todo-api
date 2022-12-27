use warp::{Filter, reply::Json};
use serde_json::{json, Value};
use crate::auth::is_auth;

pub fn todos_filter() ->  impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let todos_base = warp::path("todos");

    let list = todos_base
        .and(warp::get())
        .and(warp::path::end())
        .and(is_auth().untuple_one())
        .and_then(todo_list);
    
    let get = todos_base
        .and(warp::get())
        .and(warp::path::param())
        .and(is_auth().untuple_one())
        .and_then(todo_get);

    let create = todos_base
    .and(warp::post())
    .and(warp::body::json())
    .and(is_auth().untuple_one())
    .and_then(create_todo);

    list.or(get).or(create)
}

async fn todo_list() -> Result<Json, warp::Rejection> {
    let todos = json!([
        {"id": 1, "title": "todo 1"},
        {"id": 2, "title": "todo 2"},
        {"id": 3, "title": "todo 3"},
    ]);

    let todos = warp::reply::json(&todos);
    Ok(todos)
} 

async fn todo_get(id: i64) -> Result<Json, warp::Rejection> {
    let todo = json!(
        {"id": id, "title": format!("todo {}", id)}
    );
  
    let todo = warp::reply::json(&todo);
    Ok(todo)
} 

async fn create_todo(data: Value) -> Result<Json, warp::Rejection> {
    let todo = data;
  
    let todo = warp::reply::json(&todo);
    
    Ok(todo)
} 