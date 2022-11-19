use crate::firebase::{self, User};
use axum::{response::IntoResponse, Json};
use axum_auth::AuthBearer;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct PostParameter {}

pub async fn post_users_me(
    Json(_params): Json<PostParameter>,
    AuthBearer(token): AuthBearer,
) -> impl IntoResponse {
    let user = match firebase::get_claim(&token).await {
        Ok(user) => user,
        Err(err) => panic!("{:?}", &err),
    };
    let user: User = serde_json::from_value(user.to_owned()).unwrap();
    Json(json!(user))
}
