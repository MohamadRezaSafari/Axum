use std::collections::HashMap;
use headers::UserAgent;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use axum::http::{HeaderMap, HeaderValue};
use serde_json::{Result, Value};
use std::str;


pub async fn mirror_user_agent(headers: HeaderMap) -> String {

    let mut user_agent = String::new();
    if let Some(value) = headers.get("user-agent") {
        user_agent = value.to_str().unwrap().to_string();
    }

    user_agent
}


    