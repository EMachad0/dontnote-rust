use async_graphql::{Error, ErrorExtensions};

#[derive(Debug, thiserror::Error)]
pub enum GqlError {
    #[error("Could not find resource")]
    NotFound,
}

impl ErrorExtensions for GqlError {
    fn extend(&self) -> Error {
        Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            GqlError::NotFound => e.set("code", "NOT_FOUND"),
        })
    }
}
