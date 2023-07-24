mod graphql;
mod database;

use async_graphql::http::GraphiQLSource;
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use http::StatusCode;
use std::convert::Infallible;
use warp::http::response as HttpResponse;
use warp::{Filter, Rejection};

#[macro_use]
extern crate async_graphql;

#[macro_use]
extern crate thiserror;

#[macro_use]
extern crate tracing;

#[tokio::main]
pub async fn main() {
    let schema = graphql::schema::build_schema().await;

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (graphql::schema::Schema, async_graphql::Request)| async move {
            let resp = schema.execute(request).await;
            Ok::<_, Infallible>(GraphQLResponse::from(resp))
        },
    );

    let graphiql = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::Builder::new()
            .header("content-type", "text/html")
            .body(GraphiQLSource::build().endpoint("/").finish())
    });

    let routes = graphiql
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(GraphQLBadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
