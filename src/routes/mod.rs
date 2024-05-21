mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_header;
mod middleware_message;

use axum::{handler, http::Method, routing::{get, post}, Extension, Router};
use tower_http::cors::{Any,  CorsLayer};


use self::hello_world::hello_world;
use self::mirror_body_string::mirror_body_string;
use self::mirror_body_json::mirror_body_json;
use self::path_variables::{path_variables, hadr_coded_path};
use self::query_params::query_params;
use self::mirror_user_agent::mirror_user_agent;
use self::mirror_custom_header::mirror_custom_header;
use self::middleware_message::middleware_message;

#[derive(Clone)]
pub struct  SharedData {
    pub message: String
}


pub fn create_routes() -> Router {

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let share_data = SharedData {
        message: "Shared Data".to_owned()
    };

    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hadr_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(cors)
        .layer(Extension(share_data))
}
