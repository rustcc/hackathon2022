use crate::state::AppState;
use procmac::{ApiModel, Validator};
use serde::{Deserialize, Serialize};
use tools::validate::is_email;
use webase::{
    doc::prelude::*,
    dto::resp::{fail, ok, ApiResult},
    extractor::json::ValidJson,
};

#[derive(Debug, Deserialize, ApiModel, Validator)]
pub struct Req {
    #[note("邮箱")]
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize, ApiModel, Validator)]
pub struct Res {
    #[note("用户id")]
    #[validate(length(1, 150))]
    pub uid: String,
}

#[post(name = "邮箱注册")]
pub async fn handle(ValidJson(req): ValidJson<Req>) -> ApiResult<Res> {
    if req.email.len() > 10 {
        return fail("邮箱已经注册");
    }
    ok(Some(Res {
        uid: String::from("abc"),
    }))
}
