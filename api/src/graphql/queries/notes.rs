use entity::{note, user_workspace, workspace};
use sea_orm::{ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait};
use uuid::Uuid;

use crate::auth::CurrentUser;
use crate::context::Context;
use crate::graphql::guards::LoggedUserGuard;
use crate::graphql::types::note::{NoteList, NoteType};

#[derive(Default)]
pub struct NotesQuery;

#[Object]
impl NotesQuery {
    #[graphql(guard = "LoggedUserGuard::default()")]
    async fn notes(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(name = "workspace_id")] workspace_uuid: Uuid,
    ) -> async_graphql::Result<Vec<NoteType>> {
        let current_user = CurrentUser::from_context(ctx)?.user();
        let app_ctx = Context::from_context(ctx);
        let conn = app_ctx.database.get_connection();
        let notes: Vec<note::Model> = note::Entity::find()
            .join(JoinType::Join, note::Relation::Workspace.def())
            .join(JoinType::Join, workspace::Relation::UserWorkspace.def())
            .filter(workspace::Column::Uuid.eq(workspace_uuid))
            .filter(user_workspace::Column::UserId.eq(current_user.id))
            .all(conn)
            .await?;
        let notes = NoteList::from(notes);
        Ok(notes.into())
    }
}
