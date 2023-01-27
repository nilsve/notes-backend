use aws_sdk_dynamodb::client::fluent_builders::Query;
use aws_sdk_dynamodb::error::QueryError;
use aws_sdk_dynamodb::types::SdkError;
use serde::Serialize;
use serde_dynamo::from_item;

// pub async fn query_items<E: Serialize>(query: Query) -> Result<Vec<E>, SdkError<QueryError>> {
//     let result = query.send().await?;
//
//     if let Some(items) = result.items() {
//         return from_item(items)?;
//     }
//
//     Ok(vec![])
// }
