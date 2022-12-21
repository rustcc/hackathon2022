use std::{collections::HashMap, fmt::Display, sync::Arc};

use axum::{body::Body, http::Request, middleware::Next, response::IntoResponse};
use http::{header, Response};
use log::info;

use crate::{
    error::Error,
    middleware::{headers_to_map, reqinfo::RequestInfo},
};

#[derive(Debug)]
pub struct ResponseInfo {
    pub reqid: String,
    pub text: String,
    pub header: HashMap<String, Vec<String>>,
}

impl Display for ResponseInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{reqid: \"{}\", header: {:?}, text: \"{}\"}}",
            self.reqid, self.header, self.text,
        )
    }
}

pub async fn print_reqinfo(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, Error> {
    let reqinfo = req.extensions().get::<Arc<RequestInfo>>();
    if let Some(reqinfo) = reqinfo {
        info!("request = {}", reqinfo);
    }
    let res = next.run(req).await;
    Ok(res)
}

pub async fn print_resinfo(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, Error> {
    let reqinfo = req.extensions().get::<Arc<RequestInfo>>();
    let reqid = match reqinfo {
        Some(reqinfo) => reqinfo.reqid.clone(),
        None => String::default(),
    };

    let res = next.run(req).await;
    let (parts, body) = res.into_parts();
    let bytes = hyper::body::to_bytes(body).await?;
    let content_type = parts.headers.get(header::CONTENT_TYPE);
    let text = match content_type {
        Some(content_type) => {
            let v = content_type.to_str()?;
            if !v.to_lowercase().starts_with("text") && !v.to_lowercase().contains("json") {
                String::default()
            } else {
                std::str::from_utf8(&bytes)?.to_string()
            }
        }
        None => std::str::from_utf8(&bytes)?.to_string(),
    };
    let header = headers_to_map(&parts.headers)?;
    let resinfo = ResponseInfo {
        reqid,
        text,
        header,
    };

    info!("response = {}", resinfo);
    let body = Body::from(bytes);
    let res = Response::from_parts(parts, body);
    Ok(res)
}
