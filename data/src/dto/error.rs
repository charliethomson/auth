use serde::{Deserialize, Serialize};
use thiserror::Error;
use valuable::Valuable;

#[derive(Debug, Error, Clone, Serialize, Deserialize, Valuable)]
pub enum DtoError {
    #[error("wow!")]
    Unimplemented,
}
