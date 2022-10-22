mod validate_token;
use serde_json::Value;

#[derive(Debug)]
pub enum Error {
    FatalError(String),
    HttpError(String),
    JWKSFetchError(String),
    JwtVaridationError(String),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    user_id: String,
    name: String,
    email: String,
}

pub async fn get_claim(token: &str) -> Result<Value, Error> {
    validate_token::validate_token(token).await
}
