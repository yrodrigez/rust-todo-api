use warp::{Filter, reply::Json};
use serde_json::{json, Value};
use crate::auth::{is_auth, UserContext};

pub fn todos_filter() ->  impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let todos_base = warp::path("todos");

    let list = todos_base
        .and(warp::get())
        .and(warp::path::end())
        .and(is_auth())
        .and_then(todo_list);
    
    let get = todos_base
        .and(warp::get())
        .and(warp::path::param())
        .and(is_auth())
        .and_then(todo_get);

    let create = todos_base
    .and(warp::post())
    .and(warp::body::json())
    .and(is_auth())
    .and_then(create_todo);

    let middleware = list.or(get).or(create);
    middleware
}

async fn todo_list(_user_context: UserContext,) -> Result<Json, warp::Rejection> {
    let todos = json!([
        {"id": 1, "title": "todo 1"},
        {"id": 2, "title": "todo 2"},
        {"id": 3, "title": "todo 3"},
    ]);

    let todos = warp::reply::json(&todos);
    Ok::<_, warp::Rejection>(todos)
} 

async fn todo_get(id: i64, _user_context: UserContext) -> Result<Json, warp::Rejection> {
    let todo = json!(
        {"id": id, "title": format!("todo {}", id)}
    );
  
    let todo = warp::reply::json(&todo);
    Ok::<_, warp::Rejection>(todo)
} 

async fn create_todo(data: Value, _user_context: UserContext) -> Result<Json, warp::Rejection> {

    let todo = warp::reply::json(&data);
   
    
    Ok::<_, warp::Rejection>(todo)
} 