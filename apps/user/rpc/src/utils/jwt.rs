use crate::error::Error;
use crate::pb::user::User;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub const REFRESH_EXPIRES: i64 = 24 * 60 * 60;
const EXPIRES: i64 = 60 * 60 * 4;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug)]
pub struct JwtToken {
    pub user_id: String,
    pub token: String,
    pub refresh_token: String,
}

impl Claims {
    pub fn new(user_id: String) -> Self {
        let now = chrono::Utc::now().timestamp();
        let exp = now + EXPIRES;
        Self {
            user_id,
            exp,
            iat: now,
        }
    }
}

pub fn gen_token(user: &User, secret: &str) -> Result<JwtToken, Error> {
    let mut claims = Claims::new(user.id.clone());
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| Error::internal_with_details(format!("jwt error, {}", e)))?;
    claims.exp += REFRESH_EXPIRES;
    let refresh_token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| Error::internal_with_details(format!("jwt error, {}", e)))?;

    Ok(JwtToken {
        user_id: user.id.clone(),
        token,
        refresh_token,
    })
}
