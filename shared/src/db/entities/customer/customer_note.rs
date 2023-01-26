use crate::db::entities::customer::CUSTOMER_PK_PREFIX;
use crate::db::entities::TableEntity;
use crate::db::table_operations::{COLUMN_PARTITION_KEY, COLUMN_SORT_KEY};
use aws_sdk_dynamodb::model::AttributeValue;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerNote {
    note_id: Uuid,
    customer_id: Uuid,
    created_at: DateTime<Utc>,
    last_updated_at: DateTime<Utc>,
    title: String,
    contents: String,
}

impl CustomerNote {
    pub fn new<S: Into<String>>(customer_id: Uuid, title: S, contents: S) -> Self {
        Self {
            note_id: Uuid::new_v4(),
            customer_id,
            title: title.into(),
            contents: contents.into(),
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
        }
    }
}

const SK_PREFIX: &'static str = "note";

const COLUMN_NOTE_ID: &'static str = "note_id";
const COLUMN_NOTE_CUSTOMER_ID: &'static str = "customer_id";
const COLUMN_NOTE_TITLE: &'static str = "title";
const COLUMN_NOTE_CONTENTS: &'static str = "contents";
const COLUMN_NOTE_CREATED_AT: &'static str = "created_at";
const COLUMN_NOTE_LAST_UPDATED_AT: &'static str = "last_updated_at";

impl From<CustomerNote> for TableEntity {
    fn from(entity: CustomerNote) -> Self {
        let mut result = HashMap::new();
        result.insert(
            Cow::Borrowed(COLUMN_PARTITION_KEY),
            AttributeValue::S(format!("{}#{}", CUSTOMER_PK_PREFIX, entity.customer_id)),
        );
        result.insert(
            Cow::Borrowed(COLUMN_SORT_KEY),
            AttributeValue::S(format!("{}#{}", SK_PREFIX, entity.note_id)),
        );
        result.insert(
            Cow::Borrowed(COLUMN_NOTE_CUSTOMER_ID),
            AttributeValue::S(entity.customer_id.to_string()),
        );
        result.insert(
            Cow::Borrowed(COLUMN_NOTE_ID),
            AttributeValue::S(entity.note_id.to_string()),
        );
        result.insert(
            Cow::Borrowed(COLUMN_NOTE_TITLE),
            AttributeValue::S(entity.title),
        );
        result.insert(
            Cow::Borrowed(COLUMN_NOTE_CONTENTS),
            AttributeValue::S(entity.contents),
        );
        result.insert(
            Cow::Borrowed(COLUMN_NOTE_CREATED_AT),
            AttributeValue::S(entity.created_at.to_string()),
        );
        result.insert(
            Cow::Borrowed(COLUMN_NOTE_LAST_UPDATED_AT),
            AttributeValue::S(entity.last_updated_at.to_string()),
        );
        TableEntity(result)
    }
}

impl TryFrom<TableEntity> for CustomerNote {
    type Error = anyhow::Error;

    fn try_from(value: TableEntity) -> anyhow::Result<Self> {
        Ok(Self {
            customer_id: Uuid::parse_str(
                value
                    .get(COLUMN_NOTE_ID)
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
