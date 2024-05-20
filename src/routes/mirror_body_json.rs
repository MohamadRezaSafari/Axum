use axum::{Json};
use serde::{Serialize, Deserialize};

#derive

struct MirrorJson {
    message: String
}

pub async fn mirror_body_json(Json(body): Json<MirrorJson>) {
    dbg!(body);
    todo!()
}