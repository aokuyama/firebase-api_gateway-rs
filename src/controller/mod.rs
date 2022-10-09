mod controller;
pub mod usecase;
pub mod lambda_http;

type Router = for<'r, 's> fn(&'r str, &'s http::Method) -> Option<Box<dyn Usecase>>;

pub struct Controller {
    router: Router,
}
pub struct Output {
    pub http_status: http::StatusCode,
    pub content_type: String,
    pub body: String,
}

pub trait Usecase {
    fn invoke(&self) -> String;
}
