type Error = Box<dyn std::error::Error>;
use super::{Controller, Output};

#[derive(serde::Serialize)]
struct RequestError {
    msg: String,
    path: String,
    method: String,
    body: String,
}

impl Controller {
    pub fn new() -> Self {
        Controller{}
    }
    pub fn input(&self, path: &str, method: &http::Method, body: &str) -> Result<Output, Error> {
        let err = RequestError {
            msg: "not found".to_owned(),
            path: path.to_owned(),
            method: method.to_string(),
            body: body.to_owned(),
        };
        let body = serde_json::to_string(&err).unwrap();
        let o = Output {
            http_status: http::StatusCode::NOT_FOUND,
            content_type: "application/json".to_owned(),
            body,
        };
        Ok(o)
    }
}
