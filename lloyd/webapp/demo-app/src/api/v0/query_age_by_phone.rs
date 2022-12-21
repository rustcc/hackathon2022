use crate::state::AppState;
use procmac::{ApiModel, Validator};
use serde::{Deserialize, Serialize};
use tools::validate::is_mobile_phone;
use webase::{
    doc::prelude::*,
    dto::resp::{fail, ok, ApiResult},
    extractor::json::ValidJson,
};

#[derive(Debug, Deserialize, ApiModel, Validator)]
pub struct Req {
    #[note("手机号码")]
    #[validate(phone)]
    pub phone: String,
}

#[derive(Debug, Serialize, ApiModel, Validator)]
pub struct Res {
    #[note("年龄")]
    #[validate(range(1, 150))]
    pub age: i32,
}

#[post(name = "查询年龄")]
pub async fn handle(ValidJson(req): ValidJson<Req>) -> ApiResult<Res> {
    if req.phone.eq("13111111111") {
        return fail("手机号码不存在");
    }
    ok(Some(Res { age: 88 }))
}
