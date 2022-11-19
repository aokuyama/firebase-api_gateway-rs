use axum::{routing::post, Router};
mod health_check;
mod users_me;

pub fn new() -> Router {
    Router::new()
        .route("/health/check", post(health_check::post_health_check))
        .route("/users/me", post(users_me::post_users_me))
}
