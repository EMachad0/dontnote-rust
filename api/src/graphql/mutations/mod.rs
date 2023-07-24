mod user_register;
mod user_login;

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    user_register::UserRegisterMutation,
    // user_login::UserLoginMutation,
);