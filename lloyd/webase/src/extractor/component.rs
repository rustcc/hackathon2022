use crate::error::Error;
use async_trait::async_trait;
use axum::extract::FromRequest;
use http::{Extensions, Request};

pub mod prelude {
    pub use super::ComponentTrait;
    pub use http::Extensions;
}

pub trait ComponentTrait: Sized {
    fn injection(extensions: &Extensions) -> Result<Self, Error>;
}

#[derive(Debug, Clone, Default)]
pub struct Component<T: ComponentTrait>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for Component<T>
where
    T: ComponentTrait + Clone + Send + Sync + 'static,
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let extensions = req.extensions();
        let value = T::injection(extensions)?;
        Ok(Component(value))
    }
}
