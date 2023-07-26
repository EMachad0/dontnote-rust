use entity::{note, user_workspace, workspace};
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, JoinType, QuerySelect};

use crate::auth::CurrentUser;
use crate::context::Context;
use crate::graphql::errors::GqlError;
use crate::graphql::guards::LoggedUserGuard;
use crate::graphql::types::note::NoteType;

#[derive(InputObject)]
pub struct CreateNoteInput {
    pub title: String,
    pub content: String,
    pub color: String,
}

#[derive(Default)]
pub struct CreateNoteMutation;

#[Object]
impl CreateNoteMutation {
    #[graphql(guard = "LoggedUserGuard::default()")]
    async fn create_note(
        &self,
        ctx: &async_graphql::Context<'_>,
        input: CreateNoteInput,
        #[graphql(name = "workspace_id")] workspace_uuid: Uuid,
    ) -> async_graphql::Result<NoteType> {
        let current_user = CurrentUser::from_context(ctx)?.user();
        let app_ctx = Context::from_context(ctx);
        let conn = app_ctx.database.get_connection();
        debug!("finding workspace {:?}", workspace_uuid);
        let workspace = workspace::Entity::find()
            .join(JoinType::Join, workspace::Relation::UserWorkspace.def())
            .filter(workspace::Column::Uuid.eq(workspace_uuid.to_string()))
            .filter(user_workspace::Column::UserId.eq(current_user.id))
            .one(conn)
            .await?
            .ok_or(GqlError::NotFound)?;
        debug!("creating note");
        let note = note::ActiveModel {
            uuid: ActiveValue::Set(Uuid::new_v4().to_string()),
            title: ActiveValue::Set(input.title),
            content: ActiveValue::Set(input.content),
            color: ActiveValue::set(input.color),
            created_by_id: ActiveValue::Set(current_user.id),
            workspace_id: ActiveValue::Set(workspace.id),
            ..Default::default()
        };
        let note = note.insert(conn).await?;
        Ok(note.into())
    }
}
