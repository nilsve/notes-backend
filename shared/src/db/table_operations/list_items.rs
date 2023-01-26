use crate::db::entities::TableEntity;
use aws_sdk_dynamodb::client::fluent_builders::Query;
use aws_sdk_dynamodb::error::QueryError;
use aws_sdk_dynamodb::types::SdkError;

pub async fn query_items(query: Query) -> Result<Vec<TableEntity>, SdkError<QueryError>> {
    let result = query.send().await?;

    if let Some(items) = result.items() {
        return Ok(items.iter().map(|item| item.to_owned().into()).collect());
    }

    Ok(vec![])
}
