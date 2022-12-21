use axum::extract::FromRequestParts;
use common::validator::Validator;
use http::request::Parts;

use crate::error::Error;

pub struct ValidPath<T: Validator>(pub T);

impl<T: Validator> ValidPath<T> {}

#[axum::async_trait]
impl<S, T: Validator> FromRequestParts<S> for ValidPath<T>
where
    T: serde::de::DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let value = axum::extract::Path::<T>::from_request_parts(parts, state).await;
        let value = match value {
            Ok(value) => value,
            Err(e) => return Err(Error::ReqPathError(e)),
        };
        value.0.validate()?;
        Ok(Self(value.0))
    }
}
