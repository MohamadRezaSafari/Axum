use axum::{BoxError, Extension, Json, RequestPartsExt};
use axum::http::StatusCode;
use serde::Deserialize;
use axum::{
    Router,
    body::Body,
    routing::get,
    extract::{Request, FromRequest, FromRequestParts},
    http::{HeaderMap, request::Parts},
    async_trait,
};
use axum::body::HttpBody;
use axum::response::IntoResponse;

#[derive(Deserialize, Debug)]
pub struct  RequestUser {
    username: String,
    password: String
}

/*#[async_trait]
impl<S> FromRequestParts<S> for RequestUser
    where
        S: Send + Sync,
        S: HttpBody + Send,
        S::Data: Send,
        S::Error: Into<BoxError>
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
      if let Some(user_agent) = parts.headers.get(USER_AGENT) {
           Ok(RequestUser(user_agent.clone()))
       } else {
           Err((StatusCode::BAD_REQUEST, "`User-Agent` header is missing"))
       }

    }
}*/

pub async fn custom_json_extractor(user: RequestUser) {
    dbg!(user);
}