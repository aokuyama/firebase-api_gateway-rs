use super::Controller;
use lambda_http::{Request, IntoResponse, Response, RequestExt};

pub async fn lambda_service(controller: Controller, event: Request) -> Result<impl IntoResponse, std::convert::Infallible> {
    let pathes = event.path_parameters();
    let raw_path = event.raw_http_path();
    let path = match pathes.first("proxy") {
        Some(x) => x,
        None => &raw_path,
    };
    let body = serde_json::from_slice(&event.body()).unwrap();

    let response = match controller.input(path, event.method(), body) {
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
