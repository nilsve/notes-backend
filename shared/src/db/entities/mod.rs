pub mod customer;
mod helpers;
mod test;

use aws_sdk_dynamodb::model::AttributeValue;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Default, Debug)]
pub struct TableEntity(pub HashMap<Cow<'static, str>, AttributeValue>);

impl Deref for TableEntity {
    type Target = HashMap<Cow<'static, str>, AttributeValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<HashMap<String, AttributeValue>> for TableEntity {
    fn from(value: HashMap<String, AttributeValue>) -> Self {
        value
            .iter()
            .fold(TableEntity::default(), |mut acc, (key, value)| {
                acc.0.insert(Cow::Owned(key.to_owned()), value.clone());
                acc
            })
    }
}
