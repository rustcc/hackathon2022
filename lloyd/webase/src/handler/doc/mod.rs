pub mod list;

use std::sync::Arc;

use axum::{extract::State, Json};
use common::doc::ApiDoc;

pub async fn handle(doc: State<Arc<ApiDoc>>) -> Json<ApiDoc> {
    let doc = doc.as_ref();
    Json(doc.clone())
}
