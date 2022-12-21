use std::sync::Arc;

use axum::extract::FromRef;
use common::doc::ApiDoc;

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub doc: Arc<ApiDoc>,
}