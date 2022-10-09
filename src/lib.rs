use controller::{Usecase, Controller, usecase::HealthCheck};
pub mod controller;

pub fn controller() -> Controller {
    Controller::new(router)
}

fn router(path: &str, method: &http::Method) -> Option<Box<dyn Usecase>> {
    match path {
        "health/check" => {
            match method {
                &http::Method::POST => Some(Box::new(HealthCheck{})),
                _ => None,
            }
        },
        _ => None,
    }
}
