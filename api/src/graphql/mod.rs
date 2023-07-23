mod mutations;
mod queries;
mod types;

use async_graphql::EmptySubscription as SubscriptionRoot;
use mutations::MutationRoot;
use queries::QueryRoot;

pub type Schema = async_graphql::Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub fn build_schema() -> Schema {
    Schema::new(
        QueryRoot::default(),
        MutationRoot::default(),
        SubscriptionRoot::default(),
    )
}
