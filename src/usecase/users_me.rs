use super::{AuthError, Usecase, UsersMe};
use crate::firebase::{get_claim, User};
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
impl Usecase for UsersMe {
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
    async fn invoke(&self, _body: &Value, user: &Value) -> Value {
        let user: User = serde_json::from_value(user.to_owned()).unwrap();
        serde_json::to_value(user).unwrap()
    }
}
