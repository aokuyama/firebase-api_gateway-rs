use super::{AuthError, Controller, Status, UsersMe};
use crate::firebase::{get_claim, User};
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
impl Controller for UsersMe {
    async fn authentication(&self, token: Option<&str>) -> Result<Value, AuthError> {
        match token {
            Some(token) => {
                match get_claim(token).await {
                    Ok(user) => return Ok(user),
                    Err(_) => (), // TODO: handling fatal err
                }
            }
            None => (),
        };
        Err(AuthError::Error)
    }
    async fn input(&self, _body: &Value, user: &Value) -> (Value, Status) {
        let user: User = serde_json::from_value(user.to_owned()).unwrap();
        (serde_json::to_value(user).unwrap(), Status::Ok)
    }
}
