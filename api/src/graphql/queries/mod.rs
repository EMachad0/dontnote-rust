mod users;
mod workspaces;
mod notes;

#[derive(MergedObject, Default)]
pub struct QueryRoot(users::UsersQuery, workspaces::WorkspacesQuery);
