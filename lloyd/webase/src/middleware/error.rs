use std::sync::Arc;

use crate::error::{Error, HttpCode};
use axum::{middleware::Next, response::IntoResponse};
use http::Request;
use hyper::Body;
use log::{error, warn};

pub async fn log_error(req: Request<Body>, next: Next<Body>) -> Result<impl IntoResponse, Error> {
    let res = next.run(req).await;
    let err = res.extensions().get::<Arc<Error>>();
    if let Some(err) = err {
        if err.status() < 500 {
            warn!("{:?}", err);
        } else {
            error!("{:?}", err);
        }
    }
    Ok(res)
}
