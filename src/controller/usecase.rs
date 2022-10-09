use super::Usecase;

pub struct HealthCheck {
}
impl Usecase for HealthCheck {
    fn invoke(&self) -> String {
        "{\"health_check\": \"ok\"}".to_owned()
    }
}
