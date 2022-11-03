mod health_check;
mod users_me;
use async_trait::async_trait;
use serde_json::Value;

pub enum AuthError {
    Guest,
    Error,
}

pub struct UsersMe {}
pub struct HealthCheck {}

#[async_trait]
pub trait Usecase {
    async fn authentication(&self, token: Option<&str>) -> Result<Value, AuthError>;
    async fn invoke(&self, body: &Value, user: &Value) -> Value;
}
