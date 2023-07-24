use biscuit::jwa::SignatureAlgorithm;
use biscuit::jws::{RegisteredHeader, Secret};
use biscuit::{ClaimsSet, RegisteredClaims, JWT};
use std::string::ToString;

use entity::user;

pub const HEADER: &str = "x-dontnote-api-token";
const ISSUER: &str = "dontnote";

#[derive(Clone)]
pub struct JwksClient {
    secret: String,
}

impl JwksClient {
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
        token.payload().cloned().map(|x| Claims(x))
    }
}

pub struct Claims(ClaimsSet<()>);

impl From<&user::Model> for Claims {
    fn from(user: &user::Model) -> Self {
        Self(ClaimsSet {
            registered: RegisteredClaims {
                issuer: Some(ISSUER.to_string()),
                subject: Some(user.uuid.to_string()),
                ..Default::default()
            },
            private: (),
        })
    }
}

#[derive(Debug)]
pub struct AuthToken<'r>(&'r str);

impl AuthToken<'_> {
    pub fn as_str(&self) -> &str {
        self.0
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("missing auth token")]
    MissingToken,
    #[error("found `{0}` auth tokens, expected 1")]
    InvalidTokenQuantity(usize),
    #[error("invalid auth token")]
    InvalidToken(#[from] biscuit::errors::Error),
    #[error("missing auth subject")]
    MissingSubject,
    #[error("not logged in")]
    Unauthenticated,
}

impl AuthError {
    pub fn anyhow(self) -> anyhow::Error {
        anyhow::Error::new(self)
    }
}

#[derive(Debug)]
pub struct AuthSubject(String);

impl AuthSubject {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

mod filters {
    use warp::Filter;

    // pub fn with_auth(
        // jwks: &'static JWKS,
    // ) -> impl Filter<Extract = (Subject,), Error = Rejection> + Clone {
        // headers_cloned()
        //     .and_then(move |headers: HeaderMap<HeaderValue>| authenticate(jwks, headers))
    // }
}

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for AuthToken<'r> {
//     type Error = AuthError;
//
//     async fn from_request(
//         request: &'r rocket::Request<'_>,
//     ) -> rocket::request::Outcome<Self, Self::Error> {
//         let keys: Vec<_> = request.headers().get(HEADER).collect();
//         match keys.len() {
//             0 => rocket::request::Outcome::Failure((
//                 rocket::http::Status::BadRequest,
//                 AuthError::MissingToken,
//             )),
//             1 => rocket::request::Outcome::Success(AuthToken(keys[0])),
//             _ => rocket::request::Outcome::Failure((
//                 rocket::http::Status::BadRequest,
//                 AuthError::InvalidTokenQuantity(keys.len()),
//             )),
//         }
//     }
// }

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for AuthSubject {
//     type Error = AuthError;
//
//     async fn from_request(
//         request: &'r rocket::Request<'_>,
//     ) -> rocket::request::Outcome<Self, Self::Error> {
//         let auth = request.rocket().state::<Auth>().unwrap();
//         let token = try_outcome!(request.guard::<AuthToken>().await);
//         let claims = match auth.decode_token(token.as_str()) {
//             Ok(claims) => claims,
//             Err(e) => {
//                 return rocket::request::Outcome::Failure((
//                     rocket::http::Status::BadRequest,
//                     AuthError::InvalidToken(e),
//                 ))
//             }
//         };
//         match claims.registered.subject {
//             None => rocket::request::Outcome::Failure((
//                 rocket::http::Status::BadRequest,
//                 AuthError::MissingSubject,
//             )),
//             Some(subject) => rocket::request::Outcome::Success(AuthSubject(subject)),
//         }
//     }
// }
