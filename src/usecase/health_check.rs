use super::{AuthError, HealthCheck, Usecase};
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
impl Usecase for HealthCheck {
    async fn authentication(&self, _token: Option<&str>) -> Result<Value, AuthError> {
        Err(AuthError::Guest)
    }
    async fn invoke(&self, _body: &Value, _user: &Value) -> Value {
        serde_json::from_str("{\"health_check\": \"ok\"}").unwrap()
    }
}
