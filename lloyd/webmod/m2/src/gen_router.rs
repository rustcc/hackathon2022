
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
                api::v0::school::select::handle_route(),
                router,
                Some("模块2::v0::学校"),
            );
    router
}