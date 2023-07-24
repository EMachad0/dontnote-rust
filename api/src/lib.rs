mod auth;
mod database;
mod graphql;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    Router, Server,
};
use graphql::schema::Schema;

#[macro_use]
extern crate async_graphql;

#[macro_use]
extern crate thiserror;

#[macro_use]
extern crate tracing;

async fn graphql_handler(schema: Extension<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
pub async fn main() {
    let schema = graphql::schema::build_schema().await;

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));
    
    #[cfg(debug_assertions)]
    info!("Playground served at http://localhost:8080/");

    Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
