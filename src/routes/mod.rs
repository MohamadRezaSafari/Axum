mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;

use axum::{handler, routing::{get, post}, Router};

use self::hello_world::hello_world;
use self::mirror_body_string::mirror_body_string;
use self::mirror_body_json::mirror_body_json;
use self::path_variables::{path_variables, hadr_coded_path};


pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hadr_coded_path))
}
