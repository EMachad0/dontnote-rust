use crate::database::Database;
use crate::graphql::{
    mutations::MutationRoot, queries::QueryRoot, subscriptions::SubscriptionRoot,
};

pub type Schema = async_graphql::Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub async fn build_schema() -> Schema {
    let db = Database::new().await;

    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        SubscriptionRoot::default(),
    )
    .data(db)
    .finish()
}
