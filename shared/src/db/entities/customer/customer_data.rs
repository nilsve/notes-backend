use crate::db::entities::customer::CUSTOMER_PK_PREFIX;
use crate::db::entities::TableEntity;
use crate::db::table_operations::list_items::query_items;
use crate::db::table_operations::{
    build_query, COLUMN_PARTITION_KEY, COLUMN_SORT_KEY, GSI_INVERSE_NAME,
};
use anyhow::anyhow;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerData {
    customer_id: Uuid,
    registered_at: DateTime<Utc>,
    name: String,
}

impl CustomerData {
    pub fn new<S: Into<String>>(name: S) -> Self {
        serde::Self {
            customer_id: Uuid::new_v4(),
            registered_at: Utc::now(),
            name: name.into(),
        }
    }
}

const SK_PREFIX: &'static str = "ud";

const COLUMN_CUSTOMER_ID: &'static str = "customer_id";
const COLUMN_CUSTOMER_NAME: &'static str = "name";
const COLUMN_REGISTERED_AT: &'static str = "registered_at";

impl From<CustomerData> for TableEntity {
    fn from(entity: CustomerData) -> Self {
        let mut result = HashMap::new();
        result.insert(
            Cow::Borrowed(COLUMN_PARTITION_KEY),
            AttributeValue::S(format!("{}#{}", CUSTOMER_PK_PREFIX, entity.customer_id)),
        );
        result.insert(
            Cow::Borrowed(COLUMN_SORT_KEY),
            AttributeValue::S(format!("{}", SK_PREFIX)),
        );
        result.insert(
            Cow::Borrowed(COLUMN_CUSTOMER_ID),
            AttributeValue::S(entity.customer_id.to_string()),
        );
        result.insert(
            Cow::Borrowed(COLUMN_CUSTOMER_NAME),
            AttributeValue::S(entity.name),
        );
        result.insert(
            Cow::Borrowed(COLUMN_REGISTERED_AT),
            AttributeValue::S(entity.registered_at.to_string()),
        );
        TableEntity(result)
    }
}

impl TryFrom<TableEntity> for CustomerData {
    type Error = anyhow::Error;

    fn try_from(value: TableEntity) -> anyhow::Result<Self> {
        Ok(Self {
            customer_id: Uuid::parse_str(
                value
                    .get(COLUMN_CUSTOMER_ID)
                    .ok_or(anyhow!("Couldn't find customer_id field"))?
                    .as_s()
                    .map_err(|val| anyhow!("Couldnt parse customer_id field with val {:?}", val))?,
            )?,
            registered_at: value.get(COLUMN_REGISTERED_AT).ok_,
            name: value
                .get(COLUMN_CUSTOMER_NAME)
                .ok_or(anyhow!("Customer name not found"))?
                .as_s()
                .map_err(|_| anyhow!("Couldnt get customer name"))?
                .to_owned(),
        })
    }
}

pub async fn get_customer_datas(client: &Client) -> anyhow::Result<Vec<CustomerData>> {
    let query = build_query(&client)
        .index_name(GSI_INVERSE_NAME)
        .key_condition_expression("#sk = :sv")
        .expression_attribute_names("#sk", COLUMN_SORT_KEY)
        .expression_attribute_values(":sv", AttributeValue::S(SK_PREFIX.into()));

    let result = query_items(query)
        .await?
        .into_iter()
        .map(|e| Ok(e.try_into()?))
        .collect();

    result
}

#[cfg(test)]
mod customer_data_test {
    use crate::db::entities::customer::customer_data::CustomerData;
    use crate::db::entities::TableEntity;

    #[test]
    fn test_serialization() {
        let customer = CustomerData::new("nils");
        let entity = TableEntity::from(customer);

        let customer = CustomerData::try_from(entity).expect("Couldnt get customer");

        assert_eq!(customer.name, "nils");

        println!("{:?}", customer);
    }
}
