use aws_sdk_dynamodb::client::fluent_builders::{Query, Scan};
use aws_sdk_dynamodb::Client;
use std::env;

pub mod add_item;
pub mod list_items;

pub enum Table {
    Customer,
}

impl Table {
    fn get_name(&self) -> String {
        let table_name = match &self {
            Table::Customer => "CUSTOMER_DATA_TABLE_NAME",
        };

        if let Ok(name) = env::var(table_name) {
            name
        } else {
            for x in env::vars() {
                println!("var {}, {}", x.0, x.1);
            }
            panic!("table not found");
        }
    }
}

pub fn build_query(client: &Client, table_name: Table) -> Query {
    client.query().table_name(table_name.get_name())
}

pub fn build_scan(client: &Client, table_name: Table) -> Scan {
    client.scan().table_name(table_name.get_name())
}
