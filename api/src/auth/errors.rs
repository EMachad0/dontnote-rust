#[derive(Debug, Error)]
pub enum AuthError {
    #[error("missing auth token")]
    MissingToken,
    #[error("bad formatted auth token")]
    BadFormattedToken(#[from] http::header::ToStrError),
    #[error("invalid auth token")]
    InvalidToken(#[from] biscuit::errors::Error),
    #[error("missing auth subject")]
    MissingSubject,
    #[error("invalid auth subject")]
    InvalidSubject,
    #[error("not logged in")]
    Unauthenticated,
}
