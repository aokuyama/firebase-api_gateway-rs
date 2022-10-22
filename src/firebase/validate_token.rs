use super::Error;
use alcoholic_jwt::{token_kid, Validation, JWKS};
use serde_json::Value;

pub async fn validate_token(token: &str) -> Result<Value, Error> {
    let jwk_url = match std::env::var("JWK_URL") {
        Ok(v) => v,
        Err(_) => return Err(Error::FatalError("env JWK_URL must be set".to_owned())),
    };
    let jwk_issuer = match std::env::var("JWK_ISSUER") {
        Ok(v) => v,
        Err(_) => return Err(Error::FatalError("env JWK_ISSUER must be set".to_owned())),
    };

    let jwks = match fetch_jwks(jwk_url.as_str()).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let validations = vec![Validation::Issuer(jwk_issuer), Validation::SubjectPresent];

    let kid = match token_kid(token) {
        Ok(res) => match res {
            Some(kid) => kid,
            None => return Err(Error::JWKSFetchError("failed to decode kid".to_owned())),
        },
        Err(err) => return Err(Error::JwtVaridationError(err.to_string())),
    };

    let jwk = match jwks.find(&kid) {
        Some(jwk) => jwk,
        None => {
            return Err(Error::JWKSFetchError(
                "Specified key not found in set".to_owned(),
            ))
        }
    };

    match alcoholic_jwt::validate(token, jwk, validations) {
        Ok(jwt) => Ok(jwt.claims),
        Err(err) => Err(Error::JwtVaridationError(err.to_string())),
    }
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, Error> {
    let val: JWKS = match reqwest::get(uri).await {
        Ok(res) => match res.json().await {
            Ok(v) => v,
            Err(err) => return Err(Error::FatalError(err.to_string())),
        },
        Err(err) => {
            return Err(Error::HttpError(
                "fetch error:".to_string() + &err.to_string(),
            ))
        }
    };
    Ok(val)
}
