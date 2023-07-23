use crate::graphql::types::user::User;

#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl UsersQuery {
    async fn user(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<User>> {
        // let current_user = User::from_context(ctx)?;

        let users: Vec<User> = {
            vec![]
        };
        Ok(users)
    }
}