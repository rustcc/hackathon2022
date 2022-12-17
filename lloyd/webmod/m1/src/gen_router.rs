//
// 此文件由build.rs文件自动生成， 请勿修改
//

use crate::api;
use crate::state::AppState;
use axum::Router;
use common::doc::ApiDoc;
use webase::router::merge;

pub fn router(doc: &mut ApiDoc) -> Router<AppState> {
    let mut router = Router::new();

    router
}
