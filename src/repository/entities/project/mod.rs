use crate::repository::{DynamoRepository, TableEntity};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_dynamo::from_items;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectEntity {
    pub project_id: Uuid,
    pub registered_at: DateTime<Utc>,
    pub name: String,
}

impl TableEntity for ProjectEntity {}

pub struct ProjectRepository;

impl DynamoRepository<ProjectEntity> for ProjectRepository {
    fn get_table_env_name() -> &'static str {
        "PROJECT_DATA_TABLE_NAME"
    }
}

impl ProjectRepository {
    pub async fn get_project(
        client: &Client,
        project_id: String,
    ) -> anyhow::Result<Option<ProjectEntity>> {
        let result = client
            .query()
            .table_name(Self::get_table_name())
            .expression_attribute_names("#project_id", "project_id")
            .expression_attribute_values(":project_id", AttributeValue::S(project_id))
            .key_condition_expression("#project_id = :project_id")
            .send()
            .await?;

        if let Some(items) = result.items() {
            return Ok(from_items(items.to_vec())?
                .first()
                .map(|c: &ProjectEntity| c.clone()));
        }

        Ok(None)
    }
}
