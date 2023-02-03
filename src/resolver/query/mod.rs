use crate::dto::project::Project;
use crate::repository::entities::project::ProjectRepository;
use crate::repository::DynamoRepository;
use crate::resolver::Context;

use juniper::{graphql_object, FieldResult};

pub struct Query;

#[graphql_object(
context = Context,
)]
impl Query {
    fn apiVersion() -> &'static str {
        "1.0"
    }

    async fn project(context: &Context, project_id: String) -> FieldResult<Option<Project>> {
        Ok(ProjectRepository::get_project(&context.client, project_id)
            .await?
            .map(|entity| entity.into()))
    }

    async fn all_projects(context: &Context) -> FieldResult<Vec<Project>> {
        Ok(ProjectRepository::scan(&context.client)
            .await?
            .into_iter()
            .map(|entity| entity.into())
            .collect())
    }
}
