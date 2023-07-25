use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new(url: &str) -> Self {
        let connection = sea_orm::Database::connect(url)
            .await
            .expect("Could not connect to database");

        Database { connection }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    pub fn from_context<'ctx>(ctx: &async_graphql::Context<'ctx>) -> &'ctx Self {
        ctx.data_unchecked::<Self>()
    }
}
