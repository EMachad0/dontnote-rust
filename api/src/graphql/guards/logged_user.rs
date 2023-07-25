use crate::auth::{AuthError, CurrentUser};

#[derive(Debug, Default)]
pub struct LoggedUserGuard;

#[async_trait::async_trait]
impl async_graphql::Guard for LoggedUserGuard {
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<()> {
        let current_user = ctx.data::<CurrentUser>();
        match current_user {
            Ok(_) => Ok(()),
            Err(_) => Err(AuthError::Unauthenticated.into()),
        }
    }
}
