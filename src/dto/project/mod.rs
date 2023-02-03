use crate::dto::project_note::ProjectNote;
use crate::repository::entities::project::ProjectEntity;
use crate::repository::entities::project_note::ProjectNoteRepository;
use crate::resolver::Context;
use chrono::{DateTime, Utc};
use juniper::*;
use uuid::Uuid;

pub struct Project {
    id: String,
    name: String,
    created_at: DateTime<Utc>,
}

#[graphql_object(description = "A Project", Context = Context)]
impl Project {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn registered_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn notes(&self, context: &Context) -> Vec<ProjectNote> {
        let note_entities =
            ProjectNoteRepository::get_notes_for_project(&context.client, self.id.to_owned())
                .await
                .expect(&format!(
                    "Couldn't get project notes for project {}",
                    self.id
                ));

        note_entities
            .into_iter()
            .map(|note_entity| note_entity.into())
            .collect()
    }
}

impl From<ProjectEntity> for Project {
    fn from(value: ProjectEntity) -> Self {
        Self {
            id: value.project_id.to_string(),
            name: value.name,
            created_at: value.registered_at,
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A new project")]
pub struct NewProject {
    name: String,
}

impl From<NewProject> for ProjectEntity {
    fn from(value: NewProject) -> Self {
        Self {
            registered_at: Utc::now(),
            name: value.name,
            project_id: Uuid::new_v4(),
        }
    }
}
