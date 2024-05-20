mod hello_world;

use axum::{body::Body, routing::get, Router};

use self::hello_world::hello_world;


pub fn create_routes() -> Router<Body> {
    Router::new().route("/", get(hello_world()))
}