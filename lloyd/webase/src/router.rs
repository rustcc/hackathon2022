use axum::{routing::get, Router};
use common::doc::ApiDoc;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

use crate::{
    doc::AxumApiRoute,
    handler::{doc, health},
    middleware::reqid::RequestIdLayer,
    state::AppState,
};

pub fn router(doc: &mut ApiDoc) -> Router<AppState> {
    let router = Router::new();
    let router = router.route("/doc", get(doc::handle));
    merge(doc, health::handle_route(), router, Some("基础服务"))
}

pub fn merge<T: Clone + Send + Sync + 'static>(
    doc: &mut ApiDoc,
    mut apiroute: AxumApiRoute<T>,
    router: Router<T>,
    mod_path: Option<&str>,
) -> Router<T> {
    let router = router.route(&apiroute.api.url, apiroute.route);
    if let Some(mod_path) = mod_path {
        apiroute.api.mod_path = mod_path.to_string();
    }
    doc.apis.push(apiroute.api);
    router
}

pub fn layer(mut router: Router) -> Router {
    router = router.layer(RequestIdLayer);
    router = router.layer(
        CorsLayer::new()
            .allow_headers(AllowHeaders::any())
            .allow_origin(AllowOrigin::any())
            .allow_methods(AllowMethods::any()),
    );
    router
}
