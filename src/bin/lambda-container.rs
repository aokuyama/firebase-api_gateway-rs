use serde::Serialize;
use lambda_runtime::{LambdaEvent, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_fn = lambda_runtime::service_fn(lambda_handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct LambdaResult {
}

async fn lambda_handler(event: LambdaEvent<serde_json::Value>)  -> Result<LambdaResult, Error> {
    dbg!(event.payload.to_string());
    Ok(LambdaResult{})
}
