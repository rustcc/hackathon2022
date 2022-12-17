use crate::doc::prelude::*;
use crate::dto::resp::{data, ApiBody};
use crate::state::AppState;
use procmac::get;
use structopt::clap::crate_name;

#[get(name = "健康检查", auth = false, power = false)]
pub async fn handle() -> ApiBody<String> {
    data(Some("ok".to_string()))
}
