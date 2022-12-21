use crate::state::AppState;
use procmac::{ApiModel, Validator};
use serde::{Deserialize, Serialize};
use webase::{
    doc::prelude::*,
    dto::resp::{ok, ApiResult},
    extractor::json::ValidJson,
};

#[derive(Debug, Deserialize, ApiModel, Validator)]
pub struct Req {
    #[note("真实姓名")]
    #[serde(rename = "rName")]
    pub real_name: String,

    #[note("项")]
    #[serde(flatten)]
    pub item: Item,
}

#[derive(Debug, Serialize, Deserialize, Validator, ApiModel)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[note("昵称")]
    pub nick_name: String,

    #[serde(skip)]
    pub age: i32,
}

#[derive(Debug, Serialize, ApiModel, Validator)]
pub struct Res {
    #[note("id")]
    pub id: i64,
}

#[post(name = "适配serde")]
pub async fn handle(ValidJson(req): ValidJson<Req>) -> ApiResult<Res> {
    if req.real_name.is_empty() {
        return ok(Some(Res { id: 1 }));
    }
    ok(None)
}
