use entity::prelude::User;
use entity::user;
use sea_orm::EntityTrait;

use crate::context::Context;
use crate::graphql::types::user::{UserList, UserType};

#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl UsersQuery {
    async fn user(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<UserType>> {
        let ctx = Context::from_context(ctx);
        let conn = ctx.database.get_connection();
        let users: Vec<user::Model> = User::find().all(conn).await?;
        let users = UserList::from(users);
        Ok(users.into())
    }
}
