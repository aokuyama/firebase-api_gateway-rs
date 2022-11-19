use firebase_api_gateway::router;
use http::HeaderValue;
use http::Method;
use lambda_http::Error;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let r = router::new();
    let allow_origin = std::env::var("ACCESS_CONTROL_ALLOW_ORIGIN")
        .expect("ACCESS_CONTROL_ALLOW_ORIGIN must be set");

    let app = lambda_http::tower::ServiceBuilder::new()
        .layer(axum_aws_lambda::LambdaLayer::default())
        .layer(
            CorsLayer::new()
                .allow_origin(allow_origin.parse::<HeaderValue>().unwrap())
                .allow_methods(vec![Method::GET, Method::POST]),
        )
        .service(r);

    lambda_http::run(app).await
}
