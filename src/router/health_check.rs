use axum::{response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct PostParameter {}

pub async fn post_health_check(Json(_params): Json<PostParameter>) -> impl IntoResponse {
    Json(json!({
        "status": "ok",
    }))
}
