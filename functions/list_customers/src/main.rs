use anyhow::Error;
use lambda_http::{run, service_fn, Body, Request, Response};
use std::convert::TryFrom;

use aws_sdk_dynamodb::Client;
use shared::db::entities::customer_data::CustomerData;
use shared::db::table_operations::add_item;
use shared::db::table_operations::list_items::list_items;

async fn list_customers(_event: Request) -> anyhow::Result<Response<Body>> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let entities = list_items(&client).await?;
    let customers: Vec<CustomerData> = entities
        .into_iter()
        .filter_map(|entity| match CustomerData::try_from(entity) {
            Ok(customer) => Some(customer),
            Err(err) => {
                println!("{:?}", err);
                None
            }
        })
        .collect();

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
