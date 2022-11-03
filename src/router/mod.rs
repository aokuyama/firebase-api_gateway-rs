use crate::usecase::Usecase;
mod http_router;
type HttpRoute = for<'r, 's> fn(&'r str, &'s http::Method) -> Option<Box<dyn Usecase>>;

pub struct HttpRouter {
    route: HttpRoute,
}
#[derive(Debug)]
pub struct Output {
    pub http_status: http::StatusCode,
    pub content_type: String,
    pub body: String,
}
