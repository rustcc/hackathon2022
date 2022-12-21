use http::Request;
use std::fmt;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower_layer::Layer;
use tower_service::Service;
use ulid::Ulid;

#[derive(Debug)]
pub struct RequestId(pub String);

impl RequestId {
    fn new() -> Self {
        Self(Ulid::new().to_string())
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct RequestIdService<S> {
    inner: S,
}

impl<S> RequestIdService<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<B, S> Service<Request<B>> for RequestIdService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        let id = RequestId::new();
        req.extensions_mut().insert(Arc::new(id));
        self.inner.call(req)
    }
}

#[derive(Debug, Clone)]
pub struct RequestIdLayer;

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdService { inner }
    }
}
