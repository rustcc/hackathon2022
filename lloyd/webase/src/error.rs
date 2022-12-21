use std::{str::Utf8Error, sync::Arc};

use axum::{
    response::{IntoResponse, Response},
    BoxError, Json,
};
use http::{Method, Uri};
use hyper::StatusCode;
use log::error;
use procmac::HttpCode;
use thiserror::Error;

use crate::dto::resp::{EmptyData, JsonBody};

pub trait HttpCode {
    fn status(&self) -> u16;

    fn code(&self) -> String;

    fn tips(&self) -> String;
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LoadConfigError {
    IoError(#[from] std::io::Error),

    YamlError(#[from] serde_yaml::Error),

    TomlError(#[from] toml::de::Error),
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum WriteConfigError {
    IoError(#[from] std::io::Error),

    YamlError(#[from] serde_yaml::Error),

    TomlError(#[from] toml::ser::Error),
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LoadOrWriteConfigError {
    LoadConfigError(#[from] LoadConfigError),

    WriteConfigError(#[from] WriteConfigError),
}

#[derive(Debug, Error, HttpCode)]
#[error(transparent)]
pub enum Error {
    // client error
    #[error("bad request: {0:?}")]
    #[code(400, "1000009", "错误请求: {0:?}")]
    BadRequest(Option<BoxError>),

    #[error("no permission")]
    #[code(403, "1001000", "没有权限")]
    NoPower,

    #[error("login required")]
    #[code(401, "1001001", "需要登录")]
    NeedAuth,

    #[code(400, "1001002", "参数错误: {0}")]
    ReqJsonError(#[from] axum::extract::rejection::JsonRejection),

    #[code(400, "1001003", "参数错误: {0}")]
    ReqFormError(#[from] axum::extract::rejection::FormRejection),

    #[code(400, "1001004", "参数错误: {0}")]
    ReqPathError(#[from] axum::extract::rejection::PathRejection),

    #[code(400, "1001005", "参数错误: {0}")]
    ReqQueryError(#[from] axum::extract::rejection::QueryRejection),

    #[code(400, "1001006", "参数错误: {0}")]
    ReqHeaderError(#[from] axum::extract::rejection::TypedHeaderRejection),

    #[code(401, "1001008", "参数错误: {0}")]
    MultipartRejection(#[from] axum::extract::multipart::MultipartRejection),

    #[code(401, "1001009", "参数错误: {0}")]
    MultipartError(#[from] axum::extract::multipart::MultipartError),

    #[error("service not found")]
    #[code(404, "1001010", "服务不存在")]
    NotFound,

    #[error("miss field: {0}")]
    #[code(404, "1001011", "缺少必传字段{0}")]
    MissField(String),

    // user error
    #[error("opt fail: {0}")]
    #[code(200, "3001002", "{0}")]
    Message(String),

    #[error("invalid value: {0}")]
    #[code(200, "3001003", "{0}")]
    Validate(#[from] common::validator::ValidateError),

    // server error
    #[error("server is busy")]
    #[code(500, "2001000", "服务器繁忙")]
    ServerBusy(Option<BoxError>),

    #[code(500, "2001001", "服务器繁忙")]
    HyperError(#[from] hyper::Error),

    #[code(500, "2001002", "服务器繁忙")]
    AxumError(#[from] axum::Error),

    #[code(500, "2001003", "服务器繁忙")]
    StrFromUtf8(#[from] Utf8Error),

    #[code(500, "2001004", "服务器繁忙")]
    ToStrError(#[from] http::header::ToStrError),

    #[error("{0}")]
    #[code(500, "2001005", "服务器繁忙")]
    MissExtension(String),

    #[error("function not impl")]
    #[code(500, "2001006", "暂未实现")]
    NotImpl,

    #[code(500, "2001009", "服务器繁忙")]
    SerdeJsonError(#[from] serde_json::error::Error),

    #[error("service {0} not exist available node")]
    #[code(500, "2001014", "服务器繁忙")]
    RpcNoAvailableNode(String),

    #[code(500, "2001015", "服务器繁忙")]
    ReqwestError(#[from] reqwest::Error),

    #[code(500, "2001017", "服务器繁忙")]
    #[error("miss config")]
    MissConfig,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = self.status();
        let body = JsonBody::<EmptyData>::create(self.code(), self.tips(), None);
        let mut response = (
            StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(body),
        )
            .into_response();
        response.extensions_mut().insert(Arc::new(self));
        response
    }
}

pub async fn layer_error(method: Method, uri: Uri, err: BoxError) -> impl IntoResponse {
    error!(
        "layer error: method: {:?}, uri: {:?}, error: {:?}",
        err, method, uri
    );
    Error::ServerBusy(Some(err))
}

pub async fn handle_asset_error(
    method: Method,
    uri: Uri,
    err: std::io::Error,
) -> impl IntoResponse {
    error!("`{} {}` failed with {}", method, uri, err);
    Error::ServerBusy(Some(Box::new(err)))
}
