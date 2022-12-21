
// 
// 此文件由build.rs文件自动生成， 请勿修改
// 
    
use axum::Router;
use crate::state::AppState;
use common::doc::ApiDoc;
use webase::router::merge;
use crate::api;

pub fn router(doc: &mut ApiDoc) -> Router<AppState> {
    let mut router = Router::new();
    
    router = merge(
                doc,
                api::v0::query_age_by_phone::handle_route(),
                router,
                Some("演示App::app测试"),
            );
    router = merge(
                doc,
                api::v0::registe_by_email::handle_route(),
                router,
                Some("演示App::app测试"),
            );
    router
}