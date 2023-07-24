use entity::user;
use uuid::Uuid;

#[derive(Debug, SimpleObject)]
pub struct UserType {
    #[graphql(skip)]
    pub id: i32,
    #[graphql(name = "id")]
    pub uuid: Uuid,
    pub name: String,
    pub email: String,
    #[graphql(skip)]
    pub password: String,
}

impl From<user::Model> for UserType {
    fn from(value: user::Model) -> Self {
        Self {
            id: value.id,
            uuid: Uuid::parse_str(&value.uuid).expect("invalid model uuid"),
            name: value.name,
            email: value.email,
            password: value.password,
        }
    }
}

pub struct UserList(Vec<UserType>);

impl From<Vec<user::Model>> for UserList {
    fn from(value: Vec<user::Model>) -> Self {
        let list = value.into_iter().map(UserType::from).collect();
        Self(list)
    }
}

impl From<UserList> for Vec<UserType> {
    fn from(value: UserList) -> Self {
        value.0
    }
}
