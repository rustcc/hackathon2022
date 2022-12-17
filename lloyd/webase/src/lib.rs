use std::net::SocketAddr;

use anyhow::Result;
use axum::Router;
use handler::handle_404;
use signal::shutdown_signal;

pub mod args;
pub mod config;
pub mod doc;
pub mod dto;
pub mod enumer;
pub mod error;
pub mod extractor;
pub mod global;
pub mod handler;
pub mod log;
pub mod middleware;
pub mod router;
pub mod signal;
pub mod state;
pub mod util;

pub async fn app(port: u16, router: Router) -> Result<()> {
    let router = router.fallback(handle_404::handle);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let server =
        axum::Server::bind(&addr).serve(router.into_make_service_with_connect_info::<SocketAddr>());
    server.with_graceful_shutdown(shutdown_signal()).await?;
    Ok(())
}
