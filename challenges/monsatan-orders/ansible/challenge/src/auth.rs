use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

/// Claims embedded in the JWT payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject: the authenticated employee's username.
    pub sub: String,
    /// Expiration timestamp (Unix seconds).
    pub exp: u64,
}

/// Validates a JWT and returns the decoded [`Claims`] if the signature and
/// expiry are both valid.
pub fn validate_token(token: &str, secret: &[u8]) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let data = decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)?;
    Ok(data.claims)
}
