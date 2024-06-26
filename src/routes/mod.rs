mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_header;
mod middleware_message;
mod read_middleware_custom_header;
mod set_middleware_custom_header;
mod always_errors;
mod get_json;
mod validate_with_serde;
mod custom_json_extractor;

use axum::{handler, http::Method, middleware, routing::{get, post}, Extension, Router};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any,  CorsLayer};


use self::hello_world::hello_world;
use self::mirror_body_string::mirror_body_string;
use self::mirror_body_json::mirror_body_json;
use self::path_variables::{path_variables, hadr_coded_path};
use self::query_params::query_params;
use self::mirror_user_agent::mirror_user_agent;
use self::mirror_custom_header::mirror_custom_header;
use self::middleware_message::middleware_message;
use self::read_middleware_custom_header::read_middleware_custom_header;
use self::set_middleware_custom_header::set_middleware_custom_header;
use self::always_errors::always_errors;
use self::get_json::get_json;
use self::validate_with_serde::validate_with_serde;


#[derive(Clone)]
pub struct  SharedData {
    pub message: String
}


pub fn create_routes(database: DatabaseConnection) -> Router {

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let share_data = SharedData {
        message: "Shared Data".to_owned()
    };

    Router::new()
        .route("/read_middleware_custom_header", get(read_middleware_custom_header))
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hadr_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .route("/always_errors", get(always_errors))
        .route("/get_json", get(get_json))
        .route("/validate_with_serde", get(validate_with_serde))
        .layer(cors)
        .layer(Extension(share_data))
        .layer(Extension(database))
}
