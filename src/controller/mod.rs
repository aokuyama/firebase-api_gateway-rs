mod controller;
pub mod lambda_http;
pub mod usecase;
use async_trait::async_trait;
use serde_json::Value;

type Router = for<'r, 's> fn(&'r str, &'s http::Method) -> Option<Box<dyn Usecase>>;

pub struct Controller {
    router: Router,
}
#[derive(Debug)]
pub struct Output {
    pub http_status: http::StatusCode,
    pub content_type: String,
    pub body: String,
}

pub enum AuthError {
    Guest,
    Error,
}

#[async_trait]
pub trait Usecase {
    async fn authentication(&self, token: Option<&str>) -> Result<Value, AuthError>;
    async fn invoke(&self, body: &Value, user: &Value) -> Value;
}
