use crate::dto::project::{NewProject, Project};
use crate::dto::project_note::{NewProjectNote, ProjectNote};
use crate::repository::entities::project::{ProjectEntity, ProjectRepository};
use crate::repository::entities::project_note::{ProjectNoteEntity, ProjectNoteRepository};
use crate::repository::DynamoRepository;
use crate::resolver::Context;
use juniper::{graphql_object, FieldResult};

use std::convert::{TryFrom, TryInto};
pub struct Mutation;

#[graphql_object(
context = Context
)]
impl Mutation {
    async fn createProject(context: &Context, new_project: NewProject) -> FieldResult<Project> {
        let entity = ProjectEntity::from(new_project);
        ProjectRepository::upsert_entity(&context.client, entity.clone()).await?;

        Ok(entity.into())
    }

    async fn createProjectNote(
        context: &Context,
        new_project_note: NewProjectNote,
    ) -> FieldResult<ProjectNote> {
        let entity = ProjectNoteEntity::try_from(new_project_note)?;
        ProjectNoteRepository::upsert_entity(&context.client, entity.clone()).await?;

        Ok(entity.try_into()?)
    }
}
