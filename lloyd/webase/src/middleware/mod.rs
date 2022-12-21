use std::{collections::HashMap, str::Utf8Error};

use http::{HeaderMap, HeaderValue, Request};
use hyper::Body;

use crate::error::Error;

pub mod error;
pub mod reqid;

pub async fn body_to_string<E: From<Utf8Error> + From<hyper::Error>>(
    req: Request<Body>,
) -> Result<String, E> {
    let body = req.into_body();
    let bytes = hyper::body::to_bytes(body).await?;
    let content = std::str::from_utf8(&bytes)?;
    Ok(content.to_string())
}

pub fn headers_to_map(
    header: &HeaderMap<HeaderValue>,
) -> Result<HashMap<String, Vec<String>>, Error> {
    let mut map = HashMap::new();
    for item in header.into_iter() {
        let key = item.0.to_string();
        let value = item.1.to_str()?.to_string();
        let vec: Option<&mut Vec<String>> = map.get_mut(&key);
        match vec {
            Some(vec) => vec.push(value),
            None => {
                let vec = vec![value];
                map.insert(key, vec);
            }
        };
    }
    Ok(map)
}
