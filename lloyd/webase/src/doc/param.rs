use std::collections::HashMap;

use axum::extract::State;
use common::{validator::Validator, doc::ApiModel};
use serde::{Deserialize, Serialize};

use crate::{
    dto::resp::{ApiResult, JsonBody},
    extractor::multipart::{FromMultipart, ValidMultipart},
};

use super::model::ApiModelTrait;

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiParamTypeEnum {
    Path,
    Query,
    Header,
    Body(String),
    None,
}

pub trait ApiParamType {
    fn api_param_type() -> ApiParamTypeEnum;

    fn api_type_id() -> Option<String>;

    fn api_models() -> HashMap<String, ApiModel>;
}

impl<T: ApiModelTrait + Serialize> ApiParamType for JsonBody<T> {
    fn api_param_type() -> ApiParamTypeEnum {
        ApiParamTypeEnum::Body(mime::APPLICATION_JSON.to_string())
    }

    fn api_models() -> HashMap<String, ApiModel> {
        let mut map = HashMap::new();
        Self::api_grow_models(&mut map);
        map
    }

    fn api_type_id() -> Option<String> {
        Self::api_model_id()
    }
}

impl<T: ApiModelTrait> ApiParamType for axum::Json<T> {
    fn api_param_type() -> ApiParamTypeEnum {
        ApiParamTypeEnum::Body(mime::APPLICATION_JSON.to_string())
    }

    fn api_models() -> HashMap<String, ApiModel> {
        let mut map = HashMap::new();
        Self::api_grow_models(&mut map);
        map
    }

    fn api_type_id() -> Option<String> {
        Self::api_model_id()
    }
}

impl<T: ApiModelTrait + Validator> ApiParamType for crate::extractor::json::ValidJson<T> {
    fn api_param_type() -> ApiParamTypeEnum {
        ApiParamTypeEnum::Body(mime::APPLICATION_JSON.to_string())
    }

    fn api_models() -> HashMap<String, ApiModel> {
        let mut map = HashMap::new();
        T::api_grow_models(&mut map);
        map
    }

    fn api_type_id() -> Option<String> {
        T::api_model_id()
    }
}
impl<T: ApiModelTrait + Validator> ApiParamType for crate::extractor::form::ValidForm<T> {
    fn api_param_type() -> ApiParamTypeEnum {
        ApiParamTypeEnum::Body(mime::APPLICATION_WWW_FORM_URLENCODED.to_string())
    }

    fn api_models() -> HashMap<String, ApiModel> {
        let mut map = HashMap::new();
        T::api_grow_models(&mut map);
        map
    }

    fn api_type_id() -> Option<String> {
        T::api_model_id()
    }
}

impl<T: ApiModelTrait + Validator> ApiParamType for crate::extractor::header::ValidHeader<T> {
    fn api_param_type() -> ApiParamTypeEnum {
        ApiParamTypeEnum::Header
    }

    fn api_models() -> HashMap<String, ApiModel> {
        let mut map = HashMap::new();
        T::api_grow_models(&mut map);
        map
    }

    fn api_type_id() -> Option<String> {
        T::api_model_id()
    }
}

impl<T: ApiModelTrait + Validator> ApiParamType for crate::extractor::query::ValidQuery<T> {
    fn api_param_type() -> ApiParamTypeEnum {
        ApiParamTypeEnum::Query
    }

    fn api_models() -> HashMap<String, ApiModel> {
        let mut map = HashMap::new();
        T::api_grow_models(&mut map);
        map
    }

    fn api_type_id() -> Option<String> {
        T::api_model_id()
    }
}

impl<T: ApiModelTrait + Validator> ApiParamType for crate::extractor::path::ValidPath<T> {
    fn api_param_type() -> ApiParamTypeEnum {
        ApiParamTypeEnum::Path
    }

    fn api_models() -> HashMap<String, ApiModel> {
        let mut map = HashMap::new();
        T::api_grow_models(&mut map);
        map
    }

    fn api_type_id() -> Option<String> {
        T::api_model_id()
    }
}

impl<T: ApiModelTrait + Serialize> ApiParamType for ApiResult<T> {
    fn api_param_type() -> ApiParamTypeEnum {
        ApiParamTypeEnum::Body(mime::APPLICATION_JSON.to_string())
    }

    fn api_models() -> HashMap<String, ApiModel> {
        let mut map = HashMap::new();
        Self::api_grow_models(&mut map);
        map
    }

    fn api_type_id() -> Option<String> {
        Self::api_model_id()
    }
}

impl<T> ApiParamType for super::prelude::Extension<T> {
    fn api_param_type() -> ApiParamTypeEnum {
        ApiParamTypeEnum::None
    }

    fn api_models() -> HashMap<String, ApiModel> {
        HashMap::new()
    }

    fn api_type_id() -> Option<String> {
        None
    }
}

impl<T> ApiParamType for super::prelude::OptExtension<T> {
    fn api_param_type() -> ApiParamTypeEnum {
        ApiParamTypeEnum::None
    }

    fn api_models() -> HashMap<String, ApiModel> {
        HashMap::new()
    }

    fn api_type_id() -> Option<String> {
        None
    }
}

impl<T> ApiParamType for axum::Extension<T> {
    fn api_param_type() -> ApiParamTypeEnum {
        ApiParamTypeEnum::None
    }

    fn api_models() -> HashMap<String, ApiModel> {
        HashMap::new()
    }

    fn api_type_id() -> Option<String> {
        None
    }
}

impl<T> ApiParamType for State<T> {
    fn api_param_type() -> ApiParamTypeEnum {
        ApiParamTypeEnum::None
    }

    fn api_models() -> HashMap<String, ApiModel> {
        HashMap::new()
    }

    fn api_type_id() -> Option<String> {
        None
    }
}

impl<T: FromMultipart + Validator + ApiModelTrait> ApiParamType for ValidMultipart<T> {
    fn api_param_type() -> ApiParamTypeEnum {
        ApiParamTypeEnum::Body(mime::MULTIPART_FORM_DATA.to_string())
    }

    fn api_models() -> HashMap<String, ApiModel> {
        let mut map = HashMap::new();
        T::api_grow_models(&mut map);
        map
    }

    fn api_type_id() -> Option<String> {
        T::api_model_id()
    }
}
