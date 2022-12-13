use crate::db::entities::TableEntity;
use crate::db::table_operations::{COLUMN_PARTITION_KEY, COLUMN_SORT_KEY};
use anyhow::anyhow;
use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerData {
    customer_id: Uuid,
    registered: usize,
    name: String,
}

impl CustomerData {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            customer_id: Uuid::new_v4(),
            registered: 0,
            name: name.into(),
        }
    }
}

const PK_PREFIX: &'static str = "customer";
const SK_PREFIX: &'static str = "ud";

const COLUMN_CUSTOMER_ID: &'static str = "customer_id";
const COLUMN_CUSTOMER_NAME: &'static str = "name";

impl From<CustomerData> for TableEntity {
    fn from(entity: CustomerData) -> Self {
        let mut result = HashMap::new();
        result.insert(
            Cow::Borrowed(COLUMN_PARTITION_KEY),
            AttributeValue::S(format!("{}#{}", PK_PREFIX, entity.customer_id)),
        );
        result.insert(
            Cow::Borrowed(COLUMN_SORT_KEY),
            AttributeValue::S(format!("{}#{}", SK_PREFIX, entity.registered.to_string())),
        );
        result.insert(
            Cow::Borrowed(COLUMN_CUSTOMER_ID),
            AttributeValue::S(entity.customer_id.to_string()),
        );
        result.insert(
            Cow::Borrowed(COLUMN_CUSTOMER_NAME),
            AttributeValue::S(entity.name),
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
            registered: 0,
            name: value
                .get(COLUMN_CUSTOMER_NAME)
                .ok_or(anyhow!("Customer name not found"))?
                .as_s()
                .map_err(|_| anyhow!("Couldnt get customer name"))?
                .to_owned(),
        })
    }
}

#[cfg(test)]
mod customer_data_test {
    use crate::db::entities::customer_data::CustomerData;
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
