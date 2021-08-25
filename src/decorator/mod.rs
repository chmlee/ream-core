use crate::error::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Decorator {
    Ignore,
}

impl Decorator {
    pub fn from(raw: String) -> Result<Decorator, ReamError> {
        match raw.as_str() {
            "IGNORE" => Ok(Decorator::Ignore),
            _ => Err(ReamError::DecoratorError(DecoratorErrorType::InvalidDecorator)),
        }
    }
}
