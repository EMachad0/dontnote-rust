use config::Config;

use crate::auth::AuthClient;
use crate::database::Database;

#[derive(Debug, Clone)]
pub struct Context {
    pub db: Database,
    pub auth_client: AuthClient,
}

impl Context {
    pub async fn from_config(config: &Config) -> Self {
        let db = Database::new(&config.database.url).await;
        let auth_client = AuthClient::new(&config.auth.secret);

        Self { db, auth_client }
    }
}
