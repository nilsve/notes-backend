pub mod entities;

use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use serde::{Deserialize, Serialize};
use serde_dynamo::{from_items, to_item};
use std::collections::HashMap;
use std::env;

pub trait TableEntity: Deserialize<'static> + Serialize + Send + 'static {}

#[async_trait]
pub trait DynamoRepository<E: TableEntity> {
    fn get_table_name() -> String {
        if let Ok(name) = env::var(Self::get_table_env_name()) {
            name
        } else {
            panic!("Table not found for {}", Self::get_table_env_name());
        }
    }

    fn get_table_env_name() -> &'static str;

    async fn scan(client: &Client) -> anyhow::Result<Vec<E>> {
        let result = client
            .scan()
            .table_name(Self::get_table_name())
            .send()
            .await?;

        return if let Some(items) = result.items().map(|slice| slice.to_vec()) {
            let users: Vec<E> = from_items(items)?;
            anyhow::Ok(users)
        } else {
            Ok(Vec::new())
        };
    }

    async fn upsert_entity(client: &Client, entity: E) -> anyhow::Result<()> {
        let serialized: HashMap<String, AttributeValue> = to_item(entity)?;

        client
            .put_item()
            .table_name(Self::get_table_name())
            .set_item(Some(serialized))
            .send()
            .await?;

        Ok(())
    }
}
