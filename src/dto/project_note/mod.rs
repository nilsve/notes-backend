use crate::repository::entities::project_note::ProjectNoteEntity;
use chrono::{DateTime, Utc};
use juniper::*;
use std::convert::TryFrom;
use uuid::Uuid;

#[derive(GraphQLObject)]
#[graphql(description = "A project's note")]
pub struct ProjectNote {
    pub project_id: String,
    pub note_id: String,
    pub created_at: DateTime<Utc>,
    pub title: String,
    pub contents: String,
}

impl From<ProjectNoteEntity> for ProjectNote {
    fn from(value: ProjectNoteEntity) -> Self {
        Self {
            project_id: value.project_id.to_string(),
            note_id: value.note_id.to_string(),
            created_at: value.created_at,
            title: value.title,
            contents: value.contents,
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A new project's note")]
pub struct NewProjectNote {
    pub project_id: String,
    pub title: String,
    pub contents: String,
}

impl TryFrom<NewProjectNote> for ProjectNoteEntity {
    type Error = uuid::Error;

    fn try_from(value: NewProjectNote) -> Result<Self, Self::Error> {
        Ok(Self {
            project_id: Uuid::parse_str(&value.project_id)?,
            created_at: Utc::now(),
            title: value.title,
            contents: value.contents,
            note_id: Uuid::new_v4(),
        })
    }
}
