use async_trait::async_trait;
use axum::{body::HttpBody, extract::FromRequest, BoxError};
use common::validator::Validator;
use http::Request;
use serde::de::DeserializeOwned;

use crate::error::Error;

pub struct ValidForm<T: Validator>(pub T);

impl<T: Validator> ValidForm<T> {}

#[async_trait]
impl<B, S, T: Validator> FromRequest<S, B> for ValidForm<T>
where
    T: DeserializeOwned,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let value = axum::extract::Form::<T>::from_request(req, state).await;
        let value = match value {
            Ok(value) => value,
            Err(e) => return Err(Error::ReqFormError(e)),
        };
        value.0.validate()?;
        Ok(Self(value.0))
    }
}
