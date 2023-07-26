use entity::{user_workspace, workspace};
use sea_orm::{ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait};

use crate::auth::CurrentUser;
use crate::context::Context;
use crate::graphql::guards::LoggedUserGuard;
use crate::graphql::types::workspace::{WorkspaceList, WorkspaceType};

#[derive(Default)]
pub struct WorkspacesQuery;

#[Object]
impl WorkspacesQuery {
    #[graphql(guard = "LoggedUserGuard::default()")]
    async fn workspaces(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<Vec<WorkspaceType>> {
        let current_user = CurrentUser::from_context(ctx)?.user();
        let app_ctx = Context::from_context(ctx);
        let conn = app_ctx.database.get_connection();
        let workspaces: Vec<workspace::Model> = workspace::Entity::find()
            .join(
                JoinType::InnerJoin,
                workspace::Relation::UserWorkspace.def(),
            )
            .filter(user_workspace::Column::UserId.eq(current_user.id))
            .all(conn)
            .await?;
        let workspaces = WorkspaceList::from(workspaces);
        Ok(workspaces.into())
    }
}
