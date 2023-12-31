mod auth;
mod context;
mod database;
mod errors;
mod graphql;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    debug_handler,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router, Server,
};
use tower_http::cors::{Any, CorsLayer};

use crate::auth::CurrentUser;
use crate::context::Context;
use crate::graphql::schema::Schema;

#[macro_use]
extern crate async_graphql;
#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate tracing;

#[debug_handler]
async fn graphql_handler(
    current_user: Option<CurrentUser>,
    State(schema): State<Schema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    if let Some(current_user) = current_user {
        req = req.data(current_user);
    }
    schema.execute(req).await.into()
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
pub async fn main() {
    let config = config::Config::get();
    let ctx = Context::from_config(config).await;
    let schema = graphql::schema::build_schema(&ctx).await;
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .with_state(schema)
        .layer(Extension(ctx))
        .layer(cors);

    #[cfg(debug_assertions)]
    info!("Playground served at http://localhost:4000/");

    Server::bind(&"127.0.0.1:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
