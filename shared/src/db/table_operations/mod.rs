use aws_sdk_dynamodb::client::fluent_builders::Query;
use aws_sdk_dynamodb::Client;
use std::env;

pub mod add_item;
pub mod list_items;

pub const COLUMN_PARTITION_KEY: &'static str = "pk";
pub const COLUMN_SORT_KEY: &'static str = "sk";
pub const ENVIRONMENT_TABLE_NAME: &'static str = "DATA_TABLE_NAME";

pub const GSI_INVERSE_NAME: &'static str = "GSI_INVERSE";
pub const GSI_INVERSE_PK: &'static str = "sk";
pub const GSI_INVERSE_SK: &'static str = "pk";

pub const GSI1_NAME: &'static str = "GSI_1";
pub const GSI1_PK: &'static str = "gsi1_pk";
pub const GSI1_SK: &'static str = "gsi2_pk";

pub fn build_query(client: &Client) -> Query {
    client
        .query()
        .table_name(env::var(ENVIRONMENT_TABLE_NAME).expect("Data table name not found in env"))
}
