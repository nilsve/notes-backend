use crate::db::entities::TableEntity;
use crate::db::table_operations::{COLUMN_PARTITION_KEY, COLUMN_SORT_KEY, ENVIRONMENT_TABLE_NAME};
use aws_sdk_dynamodb::error::PutItemError;
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::Client;
use std::env;
use std::ops::Deref;

pub async fn add_item<E: Into<TableEntity>>(
    client: &Client,
    item: E,
) -> Result<(), SdkError<PutItemError>> {
    let serialized: TableEntity = item.into();

    debug_assert_eq!(serialized.contains_key(COLUMN_PARTITION_KEY), true);
    debug_assert_eq!(serialized.contains_key(COLUMN_SORT_KEY), true);

    let mut put_item = client
        .put_item()
        .table_name(env::var(ENVIRONMENT_TABLE_NAME).expect("Data table name not found in env"));

    put_item = serialized
        .deref()
        .iter()
        .fold(put_item, |put_item, (key, value)| {
            put_item.item(key.deref(), value.clone())
        });

    put_item.send().await?;

    Ok(())
}
