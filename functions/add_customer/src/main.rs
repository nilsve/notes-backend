use lambda_http::{run, service_fn, Body, Request, Response};

use aws_sdk_dynamodb::Client;
use shared::db::entities::customer_data::CustomerData;
use shared::db::table_operations::add_item;

async fn add_customer(_event: Request) -> anyhow::Result<Response<Body>> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let customer = CustomerData::new("nils");

    add_item::add_item(&client, customer.clone()).await?;

    let resp = Response::new(serde_json::to_string(&customer)?.into());
    // let resp = Response::builder()
    //     .body()
    //     .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(add_customer)).await
}
