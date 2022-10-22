pub mod controller;
pub mod firebase;
pub mod usecase;

use controller::{usecase::HealthCheck, Controller, Usecase};
use usecase::users_me::UsersMe;

pub fn controller() -> Controller {
    Controller::new(router)
}

fn router(path: &str, method: &http::Method) -> Option<Box<dyn Usecase>> {
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
