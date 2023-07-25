use anyhow::Context;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use entity::user;
use http::{request::Parts, HeaderMap, HeaderValue};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::auth::auth_client::AuthClient;
use crate::auth::errors::AuthError;
use crate::auth::HEADER;
use crate::context::Context as AppContext;
use crate::errors::AppError;

#[derive(Debug)]
pub struct AuthToken(pub String);

impl AuthToken {
    pub fn from_header(headers: &HeaderMap<HeaderValue>) -> Result<AuthToken, anyhow::Error> {
        match headers.get(HEADER) {
            None => Err(AuthError::MissingToken.into()),
            Some(token) => {
                let token = token.to_str().context("Parsing Auth Token")?;
                Ok(AuthToken(token.to_owned()))
            }
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthToken
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Self::from_header(&parts.headers).map_err(AppError::from)
    }
}

#[derive(Debug)]
pub struct AuthSubject(pub String);

impl AuthSubject {
    pub fn from_token(auth: &AuthClient, token: &str) -> Result<Self, AuthError> {
        let claims = auth.decode_token(token).map_err(AuthError::InvalidToken)?;

        match claims.0.registered.subject {
            None => Err(AuthError::MissingSubject),
            Some(subject) => Ok(AuthSubject(subject)),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthSubject
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let AuthToken(token) = AuthToken::from_request_parts(parts, state).await?;

        let ctx = parts.extensions.get::<AppContext>().unwrap();
        let auth = &ctx.auth_client;

        Self::from_token(auth, &token).map_err(AppError::from)
    }
}

#[derive(Debug)]
pub struct CurrentUser(pub user::Model);

#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let AuthSubject(subject) = AuthSubject::from_request_parts(parts, _state).await?;

        let ctx = parts.extensions.get::<AppContext>().unwrap();
        let db = &ctx.database;

        let user = {
            let conn = db.get_connection();
            user::Entity::find()
                .filter(user::Column::Uuid.eq(subject))
                .one(conn)
                .await?
        };

        match user {
            None => Err(AuthError::InvalidSubject.into()),
            Some(model) => Ok(CurrentUser(model)),
        }
    }
}
