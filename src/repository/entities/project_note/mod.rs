use crate::repository::{DynamoRepository, TableEntity};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_dynamo::from_items;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectNoteEntity {
    pub project_id: Uuid,
    pub note_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub title: String,
    pub contents: String,
}

impl TableEntity for ProjectNoteEntity {}

pub struct ProjectNoteRepository;

impl DynamoRepository<ProjectNoteEntity> for ProjectNoteRepository {
    fn get_table_env_name() -> &'static str {
        "PROJECT_NOTE_DATA_TABLE_NAME"
    }
}

impl ProjectNoteRepository {
    pub async fn get_notes_for_project(
        client: &Client,
        project_id_id: String,
    ) -> anyhow::Result<Vec<ProjectNoteEntity>> {
        let result = client
            .query()
            .table_name(Self::get_table_name())
            .expression_attribute_names("#project_id", "project_id")
            .expression_attribute_values(":project_id", AttributeValue::S(project_id_id))
            .key_condition_expression("#project_id = :project_id")
            .send()
            .await?;

        if let Some(items) = result.items() {
            return Ok(from_items(items.to_vec())?);
        }

        Ok(Vec::new())
    }
}
