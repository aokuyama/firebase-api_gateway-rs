pub mod controller;
pub mod firebase;
pub mod lambda;
pub mod router;
use controller::{Controller, HealthCheck, UsersMe};
use router::HttpRouter;

pub fn create_http_router() -> HttpRouter {
    HttpRouter::new(route)
}

fn route(path: &str, method: &http::Method) -> Option<Box<dyn Controller>> {
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
