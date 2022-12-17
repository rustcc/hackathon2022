
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
    
    router
}