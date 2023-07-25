use biscuit::{ClaimsSet, RegisteredClaims};
use entity::user;

use crate::auth::ISSUER;

pub struct Claims(pub ClaimsSet<()>);

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
