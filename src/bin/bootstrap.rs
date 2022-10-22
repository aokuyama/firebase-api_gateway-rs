use firebase_api_gateway::controller;
use lambda_http::{service_fn, Error, IntoResponse, Request};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = service_fn(lambda_service);
    lambda_http::run(handler).await?;
    Ok(())
}

async fn lambda_service(event: Request) -> Result<impl IntoResponse, std::convert::Infallible> {
    let controller = controller();
    controller::lambda_http::lambda_service(controller, event).await
}
