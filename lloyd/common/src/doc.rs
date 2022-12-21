use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::validator::ValidateType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDoc {
    pub name: String,
    pub note: String,
    pub apis: Vec<ApiOperation>,
}

impl ApiDoc {
    pub fn new(name: String, note: String) -> Self {
        ApiDoc {
            name,
            note,
            apis: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiOperation {
    // 请求方式
    pub method: String,

    // 请求url
    pub url: String,

    // 标签
    pub tags: Vec<String>,

    // 注解：中间件名称
    pub signs: Vec<String>,

    // 是否过期
    pub deprecated: bool,

    // 接口名称
    pub name: String,

    // 接口描述
    pub note: String,

    // 包名
    pub crate_name: String,

    // 模块名称
    pub mod_name: String,

    // 模块路径
    pub mod_path: String,

    // 路径参数
    pub path_in: Vec<ApiModel>,

    // query参数
    pub query_in: Vec<ApiModel>,

    // 请求头参数
    pub header_in: Vec<ApiModel>,

    // 请求头参数
    pub body_in: Option<ApiBodyParam>,

    // 响应体参数
    pub body_out: Option<ApiBodyParam>,

    // 操作类型: 增，删，改，查
    pub opt: String,

    // 是否需要授权
    pub auth: bool,

    // 是否需要权限
    pub power: bool,
}

impl PartialEq for ApiOperation {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiBodyParam {
    pub content_type: String,
    pub model_id: String,
    pub models: HashMap<String, ApiModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiModel {
    pub members: Vec<ApiMember>,
    pub note: String,
    pub ty: ApiFieldType,
    pub model_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiMember {
    pub validate: Vec<ValidateType>,
    pub note: String,
    pub default: bool,
    pub serialize: Option<ApiField>,
    pub deserialize: Option<ApiField>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiField {
    pub name: String,
    pub ty: ApiFieldType,
    pub inner: Option<String>,
    pub option: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ApiFieldType {
    Number,
    String,
    ConstString(String),
    EmptyArray,
    IsomerismArray,
    Object,
    List(Box<ApiFieldType>),
    Set(Box<ApiFieldType>),
    Map(Box<ApiFieldType>),
    Bool,
    Enumer,
    File,
    TagStuct { tag: String, name: String },
    TagEnumer { tag: String, content: String },
}

impl Display for ApiFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiFieldType::Number => write!(f, "Number"),
            ApiFieldType::String => write!(f, "String"),
            ApiFieldType::ConstString(value) => write!(f, "ConstString({})", value),
            ApiFieldType::EmptyArray => write!(f, "EmptyArray"),
            ApiFieldType::IsomerismArray => write!(f, "IsomerismArray"),
            ApiFieldType::Object => write!(f, "Object"),
            ApiFieldType::List(inner) => write!(f, "List<{}>", inner),
            ApiFieldType::Set(inner) => write!(f, "Set<{}>", inner),
            ApiFieldType::Map(inner) => write!(f, "Map<String, {}>", inner),
            ApiFieldType::Bool => write!(f, "Bool"),
            ApiFieldType::Enumer => write!(f, "Enumer"),
            ApiFieldType::File => write!(f, "File"),
            ApiFieldType::TagStuct { tag, name } => {
                write!(f, "TagStuct(tag={}, value={})", tag, name)
            }
            ApiFieldType::TagEnumer { tag, content } => {
                write!(f, "TagEnumer(tag={}, content={})", tag, content)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiParamTypeEnum {
    Path,
    Query,
    Header,
    Body(String),
    None,
}
