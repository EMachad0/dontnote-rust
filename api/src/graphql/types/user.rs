use uuid::Uuid;

#[derive(Debug, SimpleObject)]
pub struct User {
    #[graphql(skip)]
    pub id: i32,
    #[graphql(name = "id")]
    pub uuid: Uuid,
    pub name: String,
    pub email: String,
    #[graphql(skip)]
    pub password: String,
}
