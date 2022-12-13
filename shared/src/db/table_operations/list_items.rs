use crate::db::entities::TableEntity;
use crate::db::table_operations::ENVIRONMENT_TABLE_NAME;
use aws_sdk_dynamodb::error::{QueryError, ScanError};
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::Client;
use std::env;

pub async fn list_items(client: &Client) -> Result<Vec<TableEntity>, SdkError<ScanError>> {
    let result = client
        .scan()
        .table_name(env::var(ENVIRONMENT_TABLE_NAME).expect("Data table name not found in env"))
        .send()
        .await?;

    if let Some(items) = result.items() {
        return Ok(items.iter().map(|item| item.to_owned().into()).collect());
    }

    Ok(vec![])
}
