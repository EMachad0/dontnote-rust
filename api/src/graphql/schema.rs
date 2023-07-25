use crate::context::Context;
use crate::graphql::{
    mutations::MutationRoot, queries::QueryRoot, subscriptions::SubscriptionRoot,
};

pub type Schema = async_graphql::Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub async fn build_schema(ctx: &Context) -> Schema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        SubscriptionRoot::default(),
    )
    .data(ctx.clone())
    .finish()
}
