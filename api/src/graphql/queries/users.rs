use entity::prelude::User;
use entity::user;
use sea_orm::EntityTrait;

use crate::auth::CurrentUser;
use crate::context::Context;
use crate::graphql::guards::LoggedUserGuard;
use crate::graphql::types::user::{UserList, UserType};

#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl UsersQuery {
    #[graphql(guard = "LoggedUserGuard::default()")]
    async fn users(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<UserType>> {
        let current_user = CurrentUser::from_context(ctx)?.user();
        let app_ctx = Context::from_context(ctx);
        let conn = app_ctx.database.get_connection();
        let users: Vec<user::Model> = User::find_by_id(current_user.id).all(conn).await?;
        let users = UserList::from(users);
        Ok(users.into())
    }
}
