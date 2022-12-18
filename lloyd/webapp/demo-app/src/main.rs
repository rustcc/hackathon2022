use std::sync::Arc;

use anyhow::Result;
use args::Args;
use axum::{routing::get_service, Router};
use common::doc::ApiDoc;
use log::info;
use structopt::StructOpt;
use tower_http::services::ServeDir;
use webase::{app, error::handle_asset_error};
mod api;
mod args;
mod gen_router;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    webase::log::init_log()?;
    let args = Args::from_args();

    let mut doc = ApiDoc {
        name: "demo app".into(),
        note: r#"
            xxxxxx
        "#
        .into(),
        apis: vec![],
    };

    // 合并api路由
    let mut router = Router::new();
    router = router.merge(gen_router::router(&mut doc).with_state(crate::state::AppState));
    router = router.merge(m1::gen_router::router(&mut doc).with_state(m1::state::AppState));
    router = router.merge(m2::gen_router::router(&mut doc).with_state(m2::state::AppState));
    router = router.merge(
        webase::router::router(&mut doc).with_state(webase::state::AppState {
            doc: Arc::new(doc.clone()),
        }),
    );

    // 添加静态文件路由
    let serve_dir = get_service(ServeDir::new("./")).handle_error(handle_asset_error);
    router = router.nest_service("/static/", serve_dir);

    // 添加中间件
    router = webase::router::layer(router);

    info!(
        "[inner] http://{}:{}/static/resources/dist/index.html",
        webase::util::ip::find_inner_ip(),
        args.port
    );
    info!(
        "[public] http://{}:{}/static/resources/dist/index.html",
        webase::util::ip::find_public_ip().await?,
        args.port
    );
    app(args.port, router).await?;
    Ok(())
}
