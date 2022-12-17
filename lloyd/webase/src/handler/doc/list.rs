use std::sync::Arc;

use axum::Json;
use common::doc::ApiDoc;

use crate::extractor::extension::Extension;

pub async fn handle(Extension(doc): Extension<Arc<ApiDoc>>) -> Json<ApiDoc> {
    let doc = doc.as_ref();
    Json(doc.clone())
}
