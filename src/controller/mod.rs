mod controller;
pub mod lambda_http;
pub struct Controller {

}
pub struct Output {
    pub http_status: http::StatusCode,
    pub content_type: String,
    pub body: String,
}
