mod health_check;
mod users_me;
use async_trait::async_trait;
use serde_json::Value;

pub enum Status {
    Ok,
    Created,
    NoContent,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
}
pub enum AuthError {
    Guest,
    Error,
}

pub struct UsersMe {}
pub struct HealthCheck {}

#[async_trait]
pub trait Controller {
    async fn authentication(&self, token: Option<&str>) -> Result<Value, AuthError>;
    async fn input(&self, body: &Value, user: &Value) -> (Value, Status);
}
