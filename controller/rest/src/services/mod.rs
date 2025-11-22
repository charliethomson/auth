use libbuildinfo::BuildInfo;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use valuable::Valuable;

use crate::{
    Args,
    services::core::{
        hasher::{Hasher, HasherError},
        jwt::{Jwt, JwtError},
    },
};

pub mod auth;
pub mod core;
pub mod manage;

#[derive(Debug, Error, Clone, Serialize, Deserialize, Valuable)]
pub enum ApiServicesError {
    #[error(transparent)]
    Jwt {
        #[from]
        inner_error: JwtError,
    },
    #[error(transparent)]
    Hasher {
        #[from]
        inner_error: HasherError,
    },
}

#[derive(Clone, Debug)]
pub struct ApiServices {
    pub hasher: Hasher,
    pub jwt: Jwt,
}
impl ApiServices {
    pub async fn new(args: &Args, _build_info: &BuildInfo) -> Result<Self, ApiServicesError> {
        Ok(Self {
            hasher: Hasher::new()?,
            jwt: Jwt::new(&args.signing_key)?,
        })
    }
}
