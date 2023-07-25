mod auth_client;
mod authenticate;
mod claims;
mod errors;

pub use auth_client::*;
pub use authenticate::*;
pub use errors::*;

pub const HEADER: &str = "x-dontnote-api-token";
const ISSUER: &str = "dontnote";
