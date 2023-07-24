use crate::auth::JwksClient;
use crate::database::Database;
use entity::user;
use sea_orm::prelude::*;

#[derive(Default)]
pub struct UserLoginMutation;

#[Object]
impl UserLoginMutation {
    pub async fn user_login(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(validator(email))] email: String,
        password: String,
    ) -> async_graphql::Result<bool> {
        let db = Database::from_context(ctx);
        let auth = ctx.data::<JwksClient>()?;
        let user: Option<user::Model> = {
            let conn = db.get_connection();
            user::Entity::find()
                .filter(user::Column::Email.eq(email))
                .one(conn)
                .await?
        };
        let user = user.ok_or_else(|| async_graphql::Error::new("Invalid email"))?;
        if user.password == password {
            let token = auth.encode_token(&user);
            ctx.insert_http_header(crate::auth::HEADER, token);
            Ok(true)
        } else {
            Err(async_graphql::Error::new("Invalid password"))
        }
    }
}
