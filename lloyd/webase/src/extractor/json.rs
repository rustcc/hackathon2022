use axum::{body::HttpBody, extract::FromRequest, BoxError};
use common::validator::Validator;
use http::Request;
use serde::de::DeserializeOwned;

use crate::error::Error;

pub struct ValidJson<T: Validator>(pub T);

impl<T: Validator> ValidJson<T> {}

#[axum::async_trait]
impl<B, T: Validator, S> FromRequest<S, B> for ValidJson<T>
where
    T: DeserializeOwned,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let value = axum::extract::Json::<T>::from_request(req, state).await;
        let value = match value {
            Ok(value) => value,
            Err(e) => return Err(Error::ReqJsonError(e)),
        };
        value.0.validate()?;
        Ok(Self(value.0))
    }
}
