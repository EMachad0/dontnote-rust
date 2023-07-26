use entity::{user_workspace, workspace};
use sea_orm::prelude::*;
use sea_orm::ActiveValue;

use crate::auth::CurrentUser;
use crate::context::Context;
use crate::graphql::guards::LoggedUserGuard;
use crate::graphql::types::workspace::WorkspaceType;

#[derive(InputObject)]
pub struct CreateWorkspaceInput {
    pub title: String,
}

#[derive(Default)]
pub struct CreateWorkspaceMutation;

#[Object]
impl CreateWorkspaceMutation {
    #[graphql(guard = "LoggedUserGuard::default()")]
    async fn create_workspace(
        &self,
        ctx: &async_graphql::Context<'_>,
        input: CreateWorkspaceInput,
    ) -> async_graphql::Result<WorkspaceType> {
        let current_user = CurrentUser::from_context(ctx)?.user();
        let app_ctx = Context::from_context(ctx);
        let conn = app_ctx.database.get_connection();
        let workspace = workspace::ActiveModel {
            uuid: ActiveValue::set(Uuid::new_v4().to_string()),
            title: ActiveValue::set(input.title),
            ..Default::default()
        };
        let workspace = workspace.insert(conn).await?;
        let user_workspace = user_workspace::ActiveModel {
            uuid: ActiveValue::set(Uuid::new_v4().to_string()),
            workspace_id: ActiveValue::set(workspace.id),
            user_id: ActiveValue::set(current_user.id),
            role: ActiveValue::set(Some("admin".to_string())),
            ..Default::default()
        };
        user_workspace.insert(conn).await?;
        Ok(workspace.into())
    }
}
