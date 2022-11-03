use firebase_api_gateway::create_http_router;
use firebase_api_gateway::lambda;

use lambda_http::{service_fn, Error, IntoResponse, Request};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = service_fn(lambda_service);
    lambda_http::run(handler).await?;
    Ok(())
}

async fn lambda_service(request: Request) -> Result<impl IntoResponse, std::convert::Infallible> {
    let router = create_http_router();
    lambda::http::invoke(router, request).await
}
