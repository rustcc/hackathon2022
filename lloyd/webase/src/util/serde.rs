use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::{fmt, num::ParseIntError, str::FromStr};
use thiserror::Error;

pub trait FromJsonValue: Sized {
    type Error: Sized + std::error::Error;
    fn from_json_value(value: Value) -> Result<Self, Self::Error>;
}

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

pub fn string_to_number<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromJsonValue,
{
    let opt = Option::<Value>::deserialize(de)?;
    if let Some(inner) = opt {
        let result = T::from_json_value(inner)
            .map_err(de::Error::custom)
            .map(Some)?;
        return Ok(result);
    }
    Ok(None)
}

#[derive(Debug, Error)]
pub enum NumberDeError {
    #[error("type not parse as number")]
    TypeError,

    #[error("convert value error")]
    ConvertError,

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

impl FromJsonValue for i32 {
    type Error = NumberDeError;

    fn from_json_value(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(n) => {
                let value = n.as_i64();
                match value {
                    Some(value) => Ok(value as i32),
                    None => Err(NumberDeError::ConvertError),
                }
            }
            Value::String(s) => s.parse::<i32>().map_err(NumberDeError::ParseIntError),
            _ => Err(NumberDeError::TypeError),
        }
    }
}
impl FromJsonValue for i64 {
    type Error = NumberDeError;

    fn from_json_value(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(n) => {
                let value = n.as_i64();
                match value {
                    Some(value) => Ok(value),
                    None => Err(NumberDeError::ConvertError),
                }
            }
            Value::String(s) => s.parse::<i64>().map_err(NumberDeError::ParseIntError),
            _ => Err(NumberDeError::TypeError),
        }
    }
}
impl FromJsonValue for u64 {
    type Error = NumberDeError;

    fn from_json_value(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(n) => {
                let value = n.as_u64();
                match value {
                    Some(value) => Ok(value),
                    None => Err(NumberDeError::ConvertError),
                }
            }
            Value::String(s) => s.parse::<u64>().map_err(NumberDeError::ParseIntError),
            _ => Err(NumberDeError::TypeError),
        }
    }
}
