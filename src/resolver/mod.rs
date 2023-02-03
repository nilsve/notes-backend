pub mod mutation;
pub mod query;

use crate::resolver::mutation::Mutation;
use crate::resolver::query::Query;
use aws_sdk_dynamodb::Client;
use juniper::EmptySubscription;

#[derive(Clone)]
pub struct Context {
    pub client: Client,
}

impl Context {
    pub async fn new() -> Self {
        let shared_config = aws_config::load_from_env().await;
        let client = Client::new(&shared_config);

        Self { client }
    }
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;
