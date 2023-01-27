use crate::db::entities::TableEntity;
use crate::db::table_operations::add_item::add_item;
use crate::db::table_operations::{build_scan, Table};
use crate::dto::customer::NewCustomer;
use anyhow::anyhow;
use aws_sdk_dynamodb::Client;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_dynamo::from_items;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerEntity {
    pub customer_id: Uuid,
    pub registered_at: DateTime<Utc>,
    pub name: String,
}

impl TableEntity for CustomerEntity {}

impl CustomerEntity {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            customer_id: Uuid::new_v4(),
            name: name.into(),
            registered_at: Utc::now(),
        }
    }
}

pub async fn get_customer_entities(client: &Client) -> anyhow::Result<Vec<CustomerEntity>> {
    let query = build_scan(client, Table::Customer);

    // .index_name(GSI_INVERSE_NAME)
    // .key_condition_expression("#sk = :sv")
    // .expression_attribute_names("#sk", COLUMN_SORT_KEY)
    // .expression_attribute_values(":sv", AttributeValue::S(SK_PREFIX.into()));

    let result = query.send().await?;

    // And deserialize them as strongly-typed data structures
    if let Some(items) = result.items().map(|slice| slice.to_vec()) {
        let users: Vec<CustomerEntity> = from_items(items)?;
        return anyhow::Ok(users);
    }

    Err(anyhow!("werkt niet"))
}

pub async fn create_customer(
    client: &Client,
    new_customer: NewCustomer,
) -> anyhow::Result<CustomerEntity> {
    let entity = CustomerEntity::from(new_customer);
    add_item(client, Table::Customer, &entity).await?;

    Ok(entity)
}
