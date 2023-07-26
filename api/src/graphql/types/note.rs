use entity::note;
use entity::prelude::User;
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::context::Context;
use crate::graphql::errors::GqlError;
use crate::graphql::types::user::UserType;

#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct NoteType {
    #[graphql(skip)]
    pub id: i32,
    #[graphql(name = "id")]
    pub uuid: Uuid,
    pub title: String,
    pub color: String,
    pub content: String,
    pub created_by_id: i32,
}

#[ComplexObject]
impl NoteType {
    pub async fn created_by(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<UserType> {
        let app_ctx = Context::from_context(ctx);
        let conn = app_ctx.database.get_connection();
        let user = User::find_by_id(self.created_by_id)
            .one(conn)
            .await?
            .ok_or(GqlError::NotFound)?;
        Ok(user.into())
    }
}

impl From<note::Model> for NoteType {
    fn from(value: note::Model) -> Self {
        Self {
            id: value.id,
            uuid: Uuid::parse_str(&value.uuid).expect("invalid model uuid"),
            title: value.title,
            color: value.color,
            content: value.content,
            created_by_id: value.created_by,
        }
    }
}

pub struct NoteList(Vec<NoteType>);

impl From<Vec<note::Model>> for NoteList {
    fn from(value: Vec<note::Model>) -> Self {
        let list = value.into_iter().map(NoteType::from).collect();
        Self(list)
    }
}

impl From<NoteList> for Vec<NoteType> {
    fn from(value: NoteList) -> Self {
        value.0
    }
}
