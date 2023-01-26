use lambda_http::{run, service_fn, Body, Request, Response};

use aws_sdk_dynamodb::Client;
use shared::db::entities::customer::customer_data::get_customer_datas;

async fn list_customers(_event: Request) -> anyhow::Result<Response<Body>> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let customers = get_customer_datas(&client).await?;
    let resp = Response::new(serde_json::to_string(&customers)?.into());

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(list_customers)).await
}
