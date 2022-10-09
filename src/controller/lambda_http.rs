use super::Controller;
use lambda_http::{Request, IntoResponse, Response, RequestExt, Body};

pub async fn lambda_service(controller: Controller, request: Request) -> Result<impl IntoResponse, std::convert::Infallible> {
    let pathes = request.path_parameters();
    let raw_path = request.raw_http_path();
    dbg!(&pathes);
    let path = match pathes.first("proxy") {
        Some(x) => x,
        None => &raw_path,
    };
    let json: serde_json::Value = match request.body() {
        Body::Text(text) => serde_json::from_str(text).unwrap_or(serde_json::Value::Null),
        _ => serde_json::Value::Null,
    };

    let response = match controller.input(path, request.method(), &json) {
        Ok(output) => 
            Response::builder()
            .status(output.http_status)
            .header("Content-Type", output.content_type)
            .body(output.body)
        ,
        Err(err) => 
            Response::builder()
            .status(http::StatusCode::SERVICE_UNAVAILABLE)
            .body(err.as_ref().to_string())
    };
    Ok(response.expect("failed to render response"))
}
