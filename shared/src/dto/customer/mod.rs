use crate::db::entities::customer::customer_data::CustomerEntity;
use chrono::{DateTime, Utc};
use juniper::*;
use uuid::Uuid;

#[derive(GraphQLObject)]
#[graphql(description = "A customer")]
pub struct Customer {
    id: String,
    name: String,
    registered_at: DateTime<Utc>,
}

impl From<CustomerEntity> for Customer {
    fn from(value: CustomerEntity) -> Self {
        Self {
            id: value.customer_id.to_string(),
            name: value.name,
            registered_at: value.registered_at,
        }
    }
}

// There is also a custom derive for mapping GraphQL input objects.

#[derive(GraphQLInputObject)]
#[graphql(description = "A new customer")]
pub struct NewCustomer {
    name: String,
}

impl From<NewCustomer> for CustomerEntity {
    fn from(value: NewCustomer) -> Self {
        Self {
            registered_at: Utc::now(),
            name: value.name,
            customer_id: Uuid::new_v4(),
        }
    }
}
