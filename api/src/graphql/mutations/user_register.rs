use entity::user;
use sea_orm::{ActiveModelTrait, ActiveValue};
use uuid::Uuid;

use crate::database::Database;
use crate::graphql::types::user::UserType;

#[derive(InputObject)]
pub struct UserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Default)]
pub struct UserRegisterMutation;

#[Object]
impl UserRegisterMutation {
    async fn user_register(
        &self,
        ctx: &async_graphql::Context<'_>,
        input: UserInput,
    ) -> async_graphql::Result<UserType> {
        let db = Database::from_context(ctx);
        let user: user::Model = {
            let conn = db.get_connection();
            let active_model = user::ActiveModel {
                uuid: ActiveValue::Set(Uuid::new_v4().to_string()),
                name: ActiveValue::Set(input.name),
                email: ActiveValue::Set(input.email),
                password: ActiveValue::Set(input.password),
                ..Default::default()
            };
            active_model.insert(conn).await?
        };
        Ok(user.into())
    }
}
