use async_trait::async_trait;
use axum::{
    body::HttpBody,
    extract::{multipart::Field, FromRequest, Multipart},
    BoxError,
};
use bytes::Bytes;
use common::validator::{ValidateError, Validator};
use http::Request;

use crate::{error::Error, util::axum::stream_to_file};

pub mod prelude {
    pub use super::FromField;
    pub use super::FromMultipart;
    pub use super::UploadFile;
    pub use super::ValidMultipart;
    pub use crate::error::Error;
    pub use axum::async_trait;
    pub use axum::body::Bytes;
    pub use axum::extract::Multipart;
    pub use axum::{
        body::HttpBody,
        extract::{multipart::Field, FromRequest},
    };
}

pub struct UploadConfig {
    pub path: String,
}

pub struct UploadFile {
    pub path: String,
    pub file_name: String,
    pub size: u64,
}

pub struct TextFile(pub String);

#[async_trait]
pub trait FromMultipart: Sized {
    async fn from_multipart(multipart: Multipart) -> Result<Self, Error>;
}

#[async_trait]
pub trait FromField: Sized {
    async fn from_field<'a>(field: Field<'a>) -> Result<Self, Error>;
}

pub struct ValidMultipart<T: Validator + FromMultipart>(pub T);

impl<T: Validator + FromMultipart> ValidMultipart<T> {}

#[axum::async_trait]
impl<S, B, T: Validator + FromMultipart> FromRequest<S, B> for ValidMultipart<T>
where
    B: HttpBody<Data = Bytes> + Default + Unpin + Send + 'static,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let value = axum::extract::Multipart::from_request(req, state).await;
        let value = match value {
            Ok(value) => value,
            Err(e) => return Err(Error::MultipartRejection(e)),
        };
        let t = T::from_multipart(value).await?;
        t.validate()?;
        Ok(Self(t))
    }
}

#[axum::async_trait]
impl<T: FromField> FromField for Option<T> {
    async fn from_field<'a>(field: Field<'a>) -> Result<Self, Error> {
        let t = T::from_field(field).await?;
        Ok(Some(t))
    }
}

#[axum::async_trait]
impl FromField for String {
    async fn from_field<'a>(field: Field<'a>) -> Result<Self, Error> {
        Ok(field.text().await?)
    }
}

#[axum::async_trait]
impl FromField for i64 {
    async fn from_field<'a>(field: Field<'a>) -> Result<Self, Error> {
        Ok(field
            .text()
            .await?
            .parse()
            .map_err(|e| Error::BadRequest(Some(Box::new(e))))?)
    }
}

#[axum::async_trait]
impl FromField for i32 {
    async fn from_field<'a>(field: Field<'a>) -> Result<Self, Error> {
        Ok(field
            .text()
            .await?
            .parse()
            .map_err(|e| Error::BadRequest(Some(Box::new(e))))?)
    }
}

#[axum::async_trait]
impl FromField for u64 {
    async fn from_field<'a>(field: Field<'a>) -> Result<Self, Error> {
        Ok(field
            .text()
            .await?
            .parse()
            .map_err(|e| Error::BadRequest(Some(Box::new(e))))?)
    }
}

#[axum::async_trait]
impl FromField for bool {
    async fn from_field<'a>(field: Field<'a>) -> Result<Self, Error> {
        let text = field.text().await?;
        let value;
        match text.as_str() {
            "true" => {
                value = true;
            }
            "false" => {
                value = false;
            }
            _ => {
                return Err(Error::Validate(ValidateError::Invalid(
                    "无效的布尔值".into(),
                )))
            }
        }

        Ok(value)
    }
}

#[axum::async_trait]
impl FromField for UploadFile {
    async fn from_field<'a>(field: Field<'a>) -> Result<Self, Error> {
        let config = "./upload/";
        let request_id = ulid::Ulid::new().to_string();
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            return Err(Error::MissField("".into()));
        };

        let split_names: Vec<&str> = file_name.split('.').collect();
        let mut ext = String::default();
        if let Some(inner) = split_names.get(1) {
            ext = format!(".{}", inner);
        }
        let path = format!("{}{}{}", config, request_id, ext);

        let save_result = stream_to_file(path.as_str(), field)
            .await
            .map_err(|e| Error::ServerBusy(Some(e.into())))?;
        Ok(Self {
            path,
            file_name,
            size: save_result,
        })
    }
}

#[axum::async_trait]
impl FromField for TextFile {
    async fn from_field<'a>(field: Field<'a>) -> Result<Self, Error> {
        let text = field.text().await?;
        Ok(Self(text))
    }
}
