type Error = Box<dyn std::error::Error>;
use serde_json::Value;

use super::{Controller, Output, Router};

#[derive(serde::Serialize)]
struct RequestError {
    msg: String,
    path: String,
    method: String,
    body: String,
}

impl Controller {
    pub fn new(router: Router) -> Self {
        Controller{router}
    }
    pub fn input(&self, path: &str, method: &http::Method, body: &Value) -> Result<Output, Error> {
        let output = match (self.router)(path, method) {
            Some(usecase) => {
                Output {
                    http_status: http::StatusCode::OK,
                    content_type: "application/json".to_owned(),
                    body: usecase.as_ref().invoke(),
                }
            },
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
}
