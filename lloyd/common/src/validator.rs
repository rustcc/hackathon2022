use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::enumer::EnumDesc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidateType {
    Length(String, String),
    Range(String, String),
    Enumer(Vec<EnumDesc>),
    Func(String),
    Phone,
    Email,
    Inner,
    Regex(String),
    Inspector(String),
}

impl Display for ValidateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidateType::Length(min, max) => write!(f, "Length({},{})", min, max),
            ValidateType::Range(min, max) => write!(f, "Length({},{})", min, max),
            ValidateType::Enumer(descs) => write!(
                f,
                "Enum({})",
                serde_json::to_string(descs).unwrap_or_default()
            ),
            ValidateType::Func(_) => write!(f, "Func"),
            ValidateType::Phone => write!(f, "Phone"),
            ValidateType::Email => write!(f, "Email"),
            ValidateType::Inner => write!(f, "Inner"),
            ValidateType::Regex(reg) => write!(f, "Regex({})", reg),
            ValidateType::Inspector(desc) => write!(f, "Inspector({})", desc),
        }
    }
}

#[derive(Debug, Error)]
pub enum ValidateError {
    #[error("{0}")]
    Invalid(String),
}

pub trait Validator {
    fn validate(&self) -> Result<(), ValidateError>;
}

impl<T: Validator> Validator for Option<T> {
    fn validate(&self) -> Result<(), ValidateError> {
        match self {
            Some(inner) => inner.validate(),
            None => Ok(()),
        }
    }
}

impl<T: Validator> Validator for Vec<T> {
    fn validate(&self) -> Result<(), ValidateError> {
        for item in self {
            item.validate()?;
        }
        Ok(())
    }
}
