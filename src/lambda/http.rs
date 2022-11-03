use lambda_http::{Body, IntoResponse, Request, RequestExt, Response};

use crate::router::HttpRouter;

pub async fn invoke(
    router: HttpRouter,
    request: Request,
) -> Result<impl IntoResponse, std::convert::Infallible> {
    let acao = std::env::var("ACCESS_CONTROL_ALLOW_ORIGIN")
        .expect("ACCESS_CONTROL_ALLOW_ORIGIN must be set");

    let pathes = request.path_parameters();
    let raw_path = request.raw_http_path();
    let path = match pathes.first("proxy") {
        Some(x) => x,
        None => &raw_path,
    };
    let json: serde_json::Value = match request.body() {
        Body::Text(text) => serde_json::from_str(text).unwrap_or(serde_json::Value::Null),
        _ => serde_json::Value::Null,
    };
    let token = match request.headers().get("Authorization") {
        Some(x) => Some(x.to_str().unwrap()),
        None => None,
    };

    let response = match router.input(path, request.method(), &json, token).await {
        Ok(output) => Response::builder()
            .status(output.http_status)
            .header("Content-Type", output.content_type)
            .header("Access-Control-Allow-Origin", acao)
            .body(output.body),
        Err(err) => Response::builder()
            .status(http::StatusCode::SERVICE_UNAVAILABLE)
            .body(err.as_ref().to_string()),
    };
    Ok(response.expect("failed to render response"))
}
