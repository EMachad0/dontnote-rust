use entity::user;
use sea_orm::prelude::*;

use crate::context::Context;

#[derive(Default)]
pub struct UserLoginMutation;

#[Object]
impl UserLoginMutation {
    pub async fn user_login(
        &self,
        gql_ctx: &async_graphql::Context<'_>,
        #[graphql(validator(email))] email: String,
        password: String,
    ) -> async_graphql::Result<String> {
        let ctx = Context::from_context(gql_ctx);
        let user: Option<user::Model> = {
            let conn = ctx.database.get_connection();
            user::Entity::find()
                .filter(user::Column::Email.eq(email))
                .one(conn)
                .await?
        };
        let user = user.ok_or_else(|| async_graphql::Error::new("Invalid email"))?;
        if user.password == password {
            let token = ctx.auth_client.encode_token(&user);
            Ok(token)
        } else {
            Err(async_graphql::Error::new("Invalid password"))
        }
    }
}
