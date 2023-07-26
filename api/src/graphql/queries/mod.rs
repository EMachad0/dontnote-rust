mod notes;
mod users;
mod workspaces;

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    notes::NotesQuery,
    users::UsersQuery,
    workspaces::WorkspacesQuery,
);
