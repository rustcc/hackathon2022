use async_trait::async_trait;
use axum::extract::FromRequestParts;
use common::validator::Validator;
use http::request::Parts;
use serde::de::DeserializeOwned;

use crate::error::Error;

pub struct ValidQuery<T: Validator>(pub T);

impl<T: Validator> ValidQuery<T> {}

#[async_trait]
impl<S, T: Validator> FromRequestParts<S> for ValidQuery<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let value = axum::extract::Query::<T>::from_request_parts(parts, state).await;
        let value = match value {
            Ok(value) => value,
            Err(e) => return Err(Error::ReqQueryError(e)),
        };
        value.0.validate()?;
        Ok(Self(value.0))
    }
}
