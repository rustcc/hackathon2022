use crate::state::AppState;
use axum::headers::{Header, HeaderName};
use axum::http::HeaderValue;
use procmac::{ApiModel, FromMultipart, Validator};
use serde::{Deserialize, Serialize};
use webase::extractor::header::ValidHeader;
use webase::extractor::multipart::prelude::*;
use webase::extractor::path::ValidPath;
use webase::extractor::query::ValidQuery;
use webase::{
    doc::prelude::*,
    dto::resp::{ok, ApiResult},
};

#[derive(Debug, FromMultipart, ApiModel, Validator)]
pub struct B {
    #[note("真实姓名")]
    #[serde(rename = "rName")]
    pub real_name: String,
}
#[derive(Debug, Deserialize, ApiModel, Validator)]
pub struct Q {
    #[note("sss")]
    pub q1: String,

    #[note("aaa")]
    pub q2: String,
}
#[derive(Debug, Deserialize, ApiModel, Validator)]
pub struct P {
    #[note("sss")]
    pub p1: String,

    #[note("bbb")]
    pub p2: String,
}
#[derive(Debug, Deserialize, ApiModel, Validator)]
pub struct H {
    #[note("h223")]
    pub h1: String,
}

#[derive(Debug, Serialize, ApiModel, Validator)]
pub struct Res {
    #[note("id")]
    pub id: i64,
}

static HEADER_NAME: axum::headers::HeaderName = HeaderName::from_static("h1");

impl Header for H {
    fn name() -> &'static axum::headers::HeaderName {
        &HEADER_NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let v = values.next().unwrap();
        Ok(H {
            h1: v.to_str().unwrap().to_string(),
        })
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, values: &mut E) {
        let v = HeaderValue::from_str(self.h1.as_str());
        match v {
            Ok(v) => {
                values.extend(vec![v]);
            }
            Err(e) => {
                error!("添加请求头失败：{:?}", e);
            }
        }
    }
}

#[post(name = "选择商品")]
pub async fn handle(
    ValidPath(_path): ValidPath<P>,
    ValidQuery(_query): ValidQuery<Q>,
    ValidHeader(_header): ValidHeader<H>,
    ValidMultipart(body): ValidMultipart<B>,
) -> ApiResult<Res> {
    if body.real_name.is_empty() {
        return ok(Some(Res { id: 1 }));
    }
    ok(None)
}
