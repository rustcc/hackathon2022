use axum::{extract::FromRequestParts, headers};
use common::validator::Validator;
use http::{request::Parts, HeaderValue};

use crate::error::Error;

pub struct ValidHeader<T: Validator>(pub T);

#[derive(Debug)]
pub struct HeaderValueString(pub HeaderValue);

impl<T: Validator> ValidHeader<T> {}

#[axum::async_trait]
impl<S, T: Validator> FromRequestParts<S> for ValidHeader<T>
where
    T: headers::Header,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let value = axum::extract::TypedHeader::<T>::from_request_parts(parts, state).await;
        let value = match value {
            Ok(value) => value,
            Err(e) => return Err(Error::ReqHeaderError(e)),
        };
        value.0.validate()?;
        Ok(Self(value.0))
    }
}
