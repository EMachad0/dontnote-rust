use entity::{user_workspace, workspace};
use sea_orm::prelude::*;
use sea_orm::ActiveValue;

use crate::auth::CurrentUser;
use crate::context::Context;
use crate::graphql::errors::GqlError;
use crate::graphql::guards::LoggedUserGuard;
use crate::graphql::types::workspace::WorkspaceType;

#[derive(Default)]
pub struct JoinWorkspaceMutation;

#[Object]
impl JoinWorkspaceMutation {
    #[graphql(guard = "LoggedUserGuard::default()")]
    async fn join_workspace(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(name = "workspace_id")] workspace_uuid: Uuid,
    ) -> async_graphql::Result<WorkspaceType> {
        let current_user = CurrentUser::from_context(ctx)?.user();
        let app_ctx = Context::from_context(ctx);
        let conn = app_ctx.database.get_connection();
        let workspace = workspace::Entity::find()
            .filter(workspace::Column::Uuid.eq(workspace_uuid))
            .one(conn)
            .await?
            .ok_or(GqlError::NotFound)?;
        let user_workspace = user_workspace::ActiveModel {
            uuid: ActiveValue::set(Uuid::new_v4().to_string()),
            workspace_id: ActiveValue::set(workspace.id),
            user_id: ActiveValue::set(current_user.id),
            ..Default::default()
        };
        user_workspace.insert(conn).await?;
        Ok(workspace.into())
    }
}
