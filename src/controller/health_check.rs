use super::{AuthError, Controller, HealthCheck, Status};
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
impl Controller for HealthCheck {
    async fn authentication(&self, _token: Option<&str>) -> Result<Value, AuthError> {
        Err(AuthError::Guest)
    }
    async fn input(&self, _body: &Value, _user: &Value) -> (Value, Status) {
        (
            serde_json::from_str("{\"health_check\": \"ok\"}").unwrap(),
            Status::Ok,
        )
    }
}
