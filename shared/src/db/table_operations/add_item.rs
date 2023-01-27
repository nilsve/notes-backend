use crate::db::table_operations::Table;
use aws_sdk_dynamodb::Client;
use serde::Serialize;
use serde_dynamo::to_item;
use std::collections::HashMap;

use crate::db::entities::TableEntity;
use anyhow::Result;
use aws_sdk_dynamodb::model::AttributeValue;

pub async fn add_item<E: TableEntity + Serialize>(
    client: &Client,
    table: Table,
    item: &E,
) -> Result<()> {
    let serialized: HashMap<String, AttributeValue> = to_item(item)?;

    client
        .put_item()
        .table_name(table.get_name())
        .set_item(Some(serialized))
        .send()
        .await?;

    Ok(())
}
