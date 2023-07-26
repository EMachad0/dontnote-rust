mod create_note;
mod create_workspace;
mod join_workspace;
mod user_login;
mod user_register;

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    create_note::CreateNoteMutation,
    create_workspace::CreateWorkspaceMutation,
    join_workspace::JoinWorkspaceMutation,
    user_login::UserLoginMutation,
    user_register::UserRegisterMutation,
);
