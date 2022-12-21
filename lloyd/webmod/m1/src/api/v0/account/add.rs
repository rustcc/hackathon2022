use crate::state::AppState;
use common::enumer::EnumFrom;
use procmac::{ApiModel, Validator};
use serde::{Deserialize, Serialize};
use tools::validate::{is_chinese_name, regex};
use webase::{
    doc::prelude::*,
    dto::resp::{ok, ApiResult},
    enumer::{Sex, Whether},
    extractor::json::ValidJson,
};

#[derive(Debug, Deserialize, ApiModel, Validator)]
pub struct Req {
    #[note("性别")]
    #[validate(enumer(Sex), message = "不是有效的性别值")]
    pub sex: String,

    #[validate(enumer(Whether))]
    pub student: i32,

    #[note("国家")]
    pub country: Country,

    #[note("昵称")]
    #[validate(length(1, 10))]
    #[validate(regex("\\w"))]
    pub nick_name: String,

    #[note("真实姓名")]
    #[validate(func(valid_real_name))]
    pub real_name: Option<String>,

    #[note("附加信息")]
    pub attach: Option<Attach>,

    #[validate(inner)]
    pub items: Option<Vec<Item>>,
}

#[derive(Debug, Serialize, Deserialize, ApiModel, Validator)]
pub struct Attach {
    #[note("x 坐标")]
    pub x: i32,
}

#[derive(Debug, Serialize, Deserialize, ApiModel, Validator)]
pub struct Item {
    #[note("主键")]
    #[validate(length(1, 5))]
    pub id: String,

    #[note("子项")]
    pub childs: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize, ApiModel)]
pub enum Country {
    #[note("中国")]
    China,

    #[note("英国")]
    Britain,
}

pub fn valid_real_name(real_name: &str) -> bool {
    is_chinese_name(real_name)
}

#[derive(Debug, Serialize, ApiModel, Validator)]
pub struct Res {
    #[note("id")]
    pub id: i64,
}

#[post(name = "参数校验", note = "真实姓名可以不是真实姓名")]
pub async fn handle(ValidJson(req): ValidJson<Req>) -> ApiResult<Res> {
    match req.country {
        Country::Britain => ok(None),
        _ => ok(Some(Res { id: 1 })),
    }
}
