use std::sync::Arc;

use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use liberror::AnyError;
use rand::Rng;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use valuable::Valuable;

#[derive(Debug, Clone, Serialize, Deserialize, Valuable, Error)]
pub enum HasherError {
    #[error("Unsupported algorithm")]
    Algorithm,

    #[error("B64 encoding error")]
    B64Encoding { inner_error: AnyError },

    #[error("Cryptographic error")]
    Crypto,

    #[error("Output size unexpected")]
    OutputSize { context: String },

    #[error("Duplicate parameter name encountered")]
    ParamNameDuplicated,

    #[error("Invalid parameter name")]
    ParamNameInvalid,

    #[error("Invalid parameter value")]
    ParamValueInvalid { context: String },

    #[error("Maximum number of parameters exceeded")]
    ParamsMaxExceeded,

    #[error("Invalid password")]
    Password,

    #[error("Password hash string invalid")]
    PhcStringField,

    #[error("Password hash string contains trailing data")]
    PhcStringTrailingData,

    #[error("Salt invalid")]
    SaltInvalid { context: String },

    #[error("Invalid algorithm version")]
    Version,

    #[error("Unknown hashing error: {inner_error}")]
    Unknown { inner_error: String },
}
impl From<argon2::password_hash::Error> for HasherError {
    fn from(value: argon2::password_hash::Error) -> Self {
        match value {
            argon2::password_hash::Error::Algorithm => Self::Algorithm,
            argon2::password_hash::Error::B64Encoding(error) => Self::B64Encoding {
                inner_error: error.into(),
            },
            argon2::password_hash::Error::Crypto => Self::Crypto,
            argon2::password_hash::Error::OutputSize { provided, expected } => Self::OutputSize {
                context: format!("({provided:?}; {expected:?})"),
            },
            argon2::password_hash::Error::ParamNameDuplicated => Self::ParamNameDuplicated,
            argon2::password_hash::Error::ParamNameInvalid => Self::ParamNameInvalid,
            argon2::password_hash::Error::ParamValueInvalid(invalid_value) => {
                Self::ParamValueInvalid {
                    context: format!("{:?}", invalid_value),
                }
            }
            argon2::password_hash::Error::ParamsMaxExceeded => Self::ParamsMaxExceeded,
            argon2::password_hash::Error::Password => Self::Password,
            argon2::password_hash::Error::PhcStringField => Self::PhcStringField,
            argon2::password_hash::Error::PhcStringTrailingData => Self::PhcStringTrailingData,
            argon2::password_hash::Error::SaltInvalid(invalid_value) => Self::SaltInvalid {
                context: format!("{:?}", invalid_value),
            },
            argon2::password_hash::Error::Version => Self::Version,

            _ => Self::Unknown {
                inner_error: value.to_string(),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Hasher {
    dummy_hash: Arc<String>,
    salt: Arc<SaltString>,
}

impl Hasher {
    pub fn new() -> Result<Self, HasherError> {
        let mut rng = rand::rng();
        let dummy_password_len = 18;
        let dummy_password = (0..dummy_password_len)
            .map(|_| rng.random::<char>())
            .collect::<String>();

        let salt = SaltString::generate(&mut OsRng);

        let argon = Argon2::default();
        let dummy_hash = argon.hash_password(dummy_password.as_bytes(), salt.as_salt())?;

        Ok(Self {
            dummy_hash: Arc::new(dummy_hash.to_string()),
            salt: Arc::new(salt),
        })
    }

    pub fn dummy_verification(&self, password: &str) {
        let _ = self.verify(&self.dummy_hash, password);
    }

    pub fn hash(&self, password: &str) -> Result<String, HasherError> {
        let argon = Argon2::default();
        let salt = self.salt.as_salt();
        let hash = argon.hash_password(password.as_bytes(), salt)?;
        Ok(hash.to_string())
    }

    pub fn verify(&self, hash: &str, password: &str) -> Result<(), HasherError> {
        let argon = Argon2::default();
        let hash = PasswordHash::new(hash)?;
        argon.verify_password(password.as_bytes(), &hash)?;
        Ok(())
    }
}
