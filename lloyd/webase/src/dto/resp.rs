use axum::Json;
use procmac::ApiModel;
use serde::{Deserialize, Serialize};

use crate::doc::model::ApiModelTrait;
use crate::error::Error;
use common::doc::ApiField;
use common::doc::ApiFieldType;
use common::doc::ApiMember;
use common::doc::ApiModel;

#[derive(Debug, Serialize, ApiModel, Deserialize)]
pub struct EmptyData;

#[derive(Debug, Serialize, Deserialize, ApiModel)]
pub struct JsonBody<T: Serialize + ApiModelTrait> {
    #[note("业务代码")]
    pub code: String,

    #[note("提示")]
    pub message: String,

    #[note("数据")]
    pub data: Option<T>,
}

impl<T: Serialize + ApiModelTrait> JsonBody<T> {
    pub fn create(code: String, message: String, data: Option<T>) -> Self {
        JsonBody {
            code,
            message,
            data,
        }
    }

    pub fn success(&self) -> bool {
        self.code.eq("0")
    }
}
pub type ApiBody<T> = Json<JsonBody<T>>;
pub type ApiResult<T> = Result<Json<JsonBody<T>>, Error>;

pub fn ok<T>(data: Option<T>) -> ApiResult<T>
where
    T: Serialize + ApiModelTrait,
{
    let resp = JsonBody::<T>::create("0".to_string(), "success".to_string(), data);
    let json = Json(resp);
    Ok(json)
}

pub fn fail<T: Serialize + ApiModelTrait>(tips: &str) -> ApiResult<T> {
    Err(Error::Message(tips.into()))
}

pub fn data<T>(data: Option<T>) -> ApiBody<T>
where
    T: Serialize + ApiModelTrait,
{
    let resp = JsonBody::<T>::create("0".to_string(), "success".to_string(), data);
    Json(resp)
}
