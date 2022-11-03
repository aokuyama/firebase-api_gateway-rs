pub mod firebase;
pub mod lambda;
pub mod router;
pub mod usecase;
use router::HttpRouter;
use usecase::{HealthCheck, Usecase, UsersMe};

pub fn create_http_router() -> HttpRouter {
    HttpRouter::new(route)
}

fn route(path: &str, method: &http::Method) -> Option<Box<dyn Usecase>> {
    match path {
        "health/check" => match method {
            &http::Method::POST => Some(Box::new(HealthCheck {})),
            _ => None,
        },
        "users/me" => match method {
            &http::Method::POST => Some(Box::new(UsersMe {})),
            _ => None,
        },
        _ => None,
    }
}
