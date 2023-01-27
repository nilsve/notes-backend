use crate::db::table_operations::{build_scan, Table};
use anyhow::anyhow;
use aws_sdk_dynamodb::Client;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_dynamo::from_items;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerData {
    customer_id: Uuid,
    registered_at: DateTime<Utc>,
    name: String,
}

impl CustomerData {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            customer_id: Uuid::new_v4(),
            name: name.into(),
            registered_at: Utc::now(),
        }
    }
}

const SK_PREFIX: &'static str = "ud";

pub async fn get_customer_datas(client: &Client) -> anyhow::Result<Vec<CustomerData>> {
    let query = build_scan(&client, Table::CustomerData);

    // .index_name(GSI_INVERSE_NAME)
    // .key_condition_expression("#sk = :sv")
    // .expression_attribute_names("#sk", COLUMN_SORT_KEY)
    // .expression_attribute_values(":sv", AttributeValue::S(SK_PREFIX.into()));

    let kut = query.send().await?;

    // And deserialize them as strongly-typed data structures
    if let Some(items) = kut.items().map(|slice| slice.to_vec()) {
        let users: Vec<CustomerData> = from_items(items)?;
        return anyhow::Ok(users);
    }

    Err(anyhow!("werkt niet"))
}
