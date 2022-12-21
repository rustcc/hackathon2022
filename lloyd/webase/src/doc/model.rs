use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::{
    dto::resp::{ApiResult, JsonBody},
    extractor::{
        header::HeaderValueString,
        multipart::{TextFile, UploadFile},
    },
};
use axum::Json;
use chrono::NaiveDateTime;
use common::doc::{ApiFieldType, ApiModel};
use serde::Serialize;

pub trait ApiModelTrait {
    fn api_grow_models(map: &mut HashMap<String, ApiModel>);

    fn api_ty() -> ApiFieldType;

    fn api_model_id() -> Option<String> {
        Some(std::any::type_name::<Self>().to_string())
    }

    fn api_note() -> String {
        String::default()
    }

    fn api_is_option() -> bool {
        false
    }
}

macro_rules! impl_base_api_model {
    ($type: ident, $en: ident) => {
        impl ApiModelTrait for $type {
            fn api_grow_models(_map: &mut HashMap<String, ApiModel>) {}

            fn api_ty() -> ApiFieldType {
                ApiFieldType::$en
            }

            fn api_model_id() -> Option<String> {
                None
            }
        }
    };
}

macro_rules! impl_collection_api_model {
    ($type: ident, $en: ident) => {
        impl<T: ApiModelTrait> ApiModelTrait for $type<T> {
            fn api_grow_models(map: &mut HashMap<String, ApiModel>) {
                T::api_grow_models(map);
            }

            fn api_ty() -> ApiFieldType {
                ApiFieldType::$en(Box::new(T::api_ty()))
            }

            fn api_model_id() -> Option<String> {
                T::api_model_id()
            }
        }
    };
}

macro_rules! impl_t_api_model {
    ($type: ident) => {
        impl<T: ApiModelTrait> ApiModelTrait for $type<T> {
            fn api_grow_models(map: &mut HashMap<String, ApiModel>) {
                T::api_grow_models(map);
            }

            fn api_ty() -> ApiFieldType {
                T::api_ty()
            }

            fn api_model_id() -> Option<String> {
                T::api_model_id()
            }
        }
    };
}

macro_rules! impl_number_api_model {
    ($type: ident) => {
        impl_base_api_model!($type, Number);
    };
}

macro_rules! impl_string_api_model {
    ($type: ident) => {
        impl_base_api_model!($type, String);
    };
}

macro_rules! impl_bool_api_model {
    ($type: ident) => {
        impl_base_api_model!($type, Bool);
    };
}

macro_rules! impl_file_api_model {
    ($type: ident) => {
        impl_base_api_model!($type, File);
    };
}

impl<T: ApiModelTrait + Serialize> ApiModelTrait for ApiResult<T> {
    fn api_grow_models(map: &mut HashMap<String, ApiModel>) {
        JsonBody::<T>::api_grow_models(map);
    }

    fn api_ty() -> ApiFieldType {
        JsonBody::<T>::api_ty()
    }

    fn api_model_id() -> Option<String> {
        JsonBody::<T>::api_model_id()
    }
}

impl<T: ApiModelTrait> ApiModelTrait for HashMap<String, T> {
    fn api_grow_models(map: &mut HashMap<String, ApiModel>) {
        T::api_grow_models(map);
    }

    fn api_ty() -> ApiFieldType {
        ApiFieldType::Map(Box::new(T::api_ty()))
    }

    fn api_model_id() -> Option<String> {
        T::api_model_id()
    }
}

impl<T: ApiModelTrait> ApiModelTrait for Option<T> {
    fn api_grow_models(map: &mut HashMap<String, ApiModel>) {
        T::api_grow_models(map);
    }

    fn api_ty() -> ApiFieldType {
        T::api_ty()
    }

    fn api_model_id() -> Option<String> {
        T::api_model_id()
    }

    fn api_is_option() -> bool {
        true
    }
}

impl_number_api_model!(u8);
impl_number_api_model!(u32);
impl_number_api_model!(i32);
impl_number_api_model!(usize);
impl_number_api_model!(i64);
impl_number_api_model!(u64);

impl_string_api_model!(String);
impl_string_api_model!(HeaderValueString);
impl_string_api_model!(NaiveDateTime);

impl_bool_api_model!(bool);

impl_file_api_model!(UploadFile);
impl_file_api_model!(TextFile);

impl_collection_api_model!(Vec, List);
impl_collection_api_model!(HashSet, Set);

impl_t_api_model!(Arc);
impl_t_api_model!(Json);
