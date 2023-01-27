





// pub async fn query_items<E: Serialize>(query: Query) -> Result<Vec<E>, SdkError<QueryError>> {
//     let result = query.send().await?;
//
//     if let Some(items) = result.items() {
//         return from_item(items)?;
//     }
//
//     Ok(vec![])
// }
