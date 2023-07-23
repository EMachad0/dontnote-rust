mod users;

#[derive(MergedObject, Default)]
pub struct QueryRoot(users::UsersQuery);
