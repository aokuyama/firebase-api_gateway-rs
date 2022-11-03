type Error = Box<dyn std::error::Error>;
use super::{HttpRoute, HttpRouter, Output};
use crate::controller::{AuthError, Controller, Status};
use serde_json::Value;

#[derive(serde::Serialize)]
struct RequestError {
    msg: String,
    path: String,
    method: String,
    body: String,
}

impl HttpRouter {
    pub fn new(route: HttpRoute) -> Self {
        HttpRouter { route }
    }
    pub async fn invoke(
        &self,
        path: &str,
        method: &http::Method,
        body: &Value,
        auth_token: Option<&str>,
    ) -> Result<Output, Error> {
        let output = match (self.route)(path, method) {
            Some(x) => self.input(x.as_ref(), body, auth_token).await,
            None => {
                let err = RequestError {
                    msg: "not found".to_owned(),
                    path: path.to_owned(),
                    method: method.to_string(),
                    body: body.to_string(),
                };
                let body = serde_json::to_string(&err).unwrap();
                Output {
                    http_status: http::StatusCode::NOT_FOUND,
                    content_type: "application/json".to_owned(),
                    body,
                }
            }
        };
        Ok(output)
    }
    async fn input(
        &self,
        controller: &dyn Controller,
        body: &Value,
        auth_token: Option<&str>,
    ) -> Output {
        match controller.authentication(auth_token).await {
            Ok(user) => {
                let (body, status) = controller.input(body, &user).await;
                self.output(body, status)
            }
            Err(e) => match e {
                AuthError::Guest => {
                    let (body, status) = controller.input(body, &Value::Null).await;
                    self.output(body, status)
                }
                AuthError::Error => Output {
                    http_status: http::StatusCode::UNAUTHORIZED,
                    content_type: "application/json".to_owned(),
                    body: "{\"msg\":\"unauthorized\"}".to_owned(),
                },
            },
        }
    }
    fn output(&self, body: Value, status: Status) -> Output {
        match serde_json::to_string(&body) {
            Ok(body) => Output {
                http_status: status_to_http(status),
                content_type: "application/json".to_owned(),
                body,
            },
            Err(_) => Output {
                http_status: http::StatusCode::INTERNAL_SERVER_ERROR,
                content_type: "application/json".to_owned(),
                body: "{\"msg\": \"error\"}".to_owned(),
            },
        }
    }
}
fn status_to_http(status: Status) -> http::StatusCode {
    match status {
        Status::Ok => http::StatusCode::OK,
        Status::Created => http::StatusCode::CREATED,
        Status::NoContent => http::StatusCode::NO_CONTENT,
        Status::BadRequest => http::StatusCode::BAD_REQUEST,
        Status::Unauthorized => http::StatusCode::UNAUTHORIZED,
        Status::Forbidden => http::StatusCode::FORBIDDEN,
        Status::NotFound => http::StatusCode::NOT_FOUND,
        Status::Conflict => http::StatusCode::CONFLICT,
    }
}
