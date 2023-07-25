use config::Config;

use crate::auth::AuthClient;
use crate::database::Database;

#[derive(Debug, Clone)]
pub struct Context {
    pub database: Database,
    pub auth_client: AuthClient,
}

impl Context {
    pub async fn from_config(config: &Config) -> Self {
        let database = Database::new(&config.database.url).await;
        let auth_client = AuthClient::new(&config.auth.secret);

        Self {
            database,
            auth_client,
        }
    }

    pub fn from_context<'ctx>(ctx: &async_graphql::Context<'ctx>) -> &'ctx Self {
        ctx.data_unchecked::<Self>()
    }
}
