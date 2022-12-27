mod todo_rest;
mod auth;

use crate::todo_rest::todos_filter;
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello_world = warp::path::end().map(|| "Hello worlds");
    let routes = hello_world.or(todos_filter());

    warp::serve (routes).run(([127,0,0,1], 8080)).await;
}