use std::{collections::HashSet, fmt::Debug};

use chrono::Utc;
use hmac::Hmac;
use jwt::{SignWithKey, VerifyWithKey};
use liberror::AnyError;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, digest::KeyInit};
use thiserror::Error;
use valuable::Valuable;

use crate::models::user::User;

#[derive(Serialize, Deserialize, Debug, Valuable, Object)]
pub struct Claims {
    #[oai(rename = "sub")]
    #[serde(rename = "sub")]
    pub user_id: i32,
    #[oai(rename = "iss")]
    #[serde(rename = "iss")]
    pub issuer: String,
    #[oai(rename = "grt")]
    #[serde(rename = "grt")]
    pub grants: Vec<String>,
    #[oai(rename = "app")]
    #[serde(rename = "app")]
    pub apps: Vec<String>,
    #[oai(rename = "iat")]
    #[serde(rename = "iat")]
    pub issued_at: u64,
    #[oai(rename = "exp")]
    #[serde(rename = "exp")]
    pub expires: u64,
}
impl Claims {
    pub fn r#for(user: &User) -> Self {
        Self {
            user_id: user.user_id,
            issuer: crate::PRODUCT_IDENTIFIER.to_string(),
            grants: user
                .grants
                .values()
                .map(|v| &v.grant_id)
                .cloned()
                // This is stupid
                .collect::<HashSet<_>>()
                .into_iter()
                .collect(),
            apps: user
                .grants
                .values()
                .map(|v| &v.application_id)
                .cloned()
                // This is stupid
                .collect::<HashSet<_>>()
                .into_iter()
                .collect(),
            issued_at: Utc::now().timestamp() as u64,
            // i dont care have a 24h token
            expires: (Utc::now() + chrono::Duration::hours(24)).timestamp() as u64,
        }
    }
}

#[derive(Debug, Error, Clone, Serialize, Deserialize, Valuable)]
pub enum JwtError {
    #[error("Failed to create signing key: {inner_error}")]
    CreateSigningKey { inner_error: AnyError },
    #[error("Failed to sign claims: {inner_error}")]
    Sign { inner_error: AnyError },
    #[error("Failed to verify token: {inner_error}")]
    Verify { inner_error: AnyError },
}

#[derive(Clone)]
pub struct Jwt {
    key: Hmac<Sha256>,
}
impl Debug for Jwt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Jwt")
    }
}

impl Jwt {
    pub fn new(key: &str) -> Result<Self, JwtError> {
        let hash =
            Hmac::new_from_slice(key.as_bytes()).map_err(|e| JwtError::CreateSigningKey {
                inner_error: e.into(),
            })?;
        Ok(Self { key: hash })
    }

    pub fn sign(&self, claims: &Claims) -> Result<String, JwtError> {
        claims.sign_with_key(&self.key).map_err(|e| JwtError::Sign {
            inner_error: e.into(),
        })
    }

    pub fn verify(&self, jwt: &str) -> Result<Claims, JwtError> {
        jwt.verify_with_key(&self.key)
            .map_err(|e| JwtError::Verify {
                inner_error: e.into(),
            })
    }
}
