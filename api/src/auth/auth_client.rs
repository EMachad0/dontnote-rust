use biscuit::jwa::SignatureAlgorithm;
use biscuit::jws::{RegisteredHeader, Secret};
use biscuit::JWT;

use crate::auth::claims::Claims;

#[derive(Debug, Clone)]
pub struct AuthClient {
    secret: String,
}

impl AuthClient {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }

    pub fn encode_token<T>(&self, claims: T) -> String
    where
        T: Into<Claims>,
    {
        let secret = Secret::bytes_from_str(&self.secret);
        let jwt = JWT::new_decoded(RegisteredHeader::default().into(), claims.into().0);
        let token = jwt.into_encoded(&secret).unwrap();
        token.unwrap_encoded().to_string()
    }

    pub fn decode_token(&self, token: &str) -> Result<Claims, biscuit::errors::Error> {
        let secret = Secret::bytes_from_str(&self.secret);
        let token = JWT::<_, biscuit::Empty>::new_encoded(token);
        let token = token.into_decoded(&secret, SignatureAlgorithm::HS256)?;
        token.payload().cloned().map(Claims)
    }
}
