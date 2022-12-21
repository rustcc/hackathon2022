use async_trait::async_trait;
use axum::extract::FromRequestParts;
use http::request::Parts;

use crate::error::Error;

#[derive(Debug, Clone, Default)]
pub struct Extension<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for Extension<T>
where
    T: Clone + Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let value = parts.extensions
            .get::<T>()
            .ok_or_else(|| {
                Error::MissExtension(format!(
                    "Extension of type `{}` was not found. Perhaps you forgot to add it? See `axum::Extension`.",
                    std::any::type_name::<T>()
                ))
            })
            .map(|x| x.clone())?;

        Ok(Extension(value))
    }
}

#[derive(Debug, Clone, Default)]
pub struct OptExtension<T>(pub Option<T>);

#[async_trait]
impl<T, S> FromRequestParts<S> for OptExtension<T>
where
    T: Clone + Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let value = parts.extensions.get::<T>();
        let value = value.cloned();
        Ok(OptExtension(value))
    }
}
