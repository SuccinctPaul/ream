#![allow(unused_variables)]
#![allow(clippy::needless_borrow)]
use std::sync::Arc;

use actix_web::{App, HttpServer, dev::ServerHandle, middleware, web};
use config::ServerConfig;
use ream_network_spec::networks::NetworkSpec;
use ream_storage::db::ReamDB;
use tracing::info;

use crate::routes::register_routers;

pub mod config;
pub mod handlers;
pub mod routes;
pub mod types;

/// Start the Beacon API server.
pub async fn start_server(
    server_config: ServerConfig,
    network_spec: Arc<NetworkSpec>,
    db: ReamDB,
) -> std::io::Result<()> {
    info!(
        "starting HTTP server on {:?}",
        server_config.http_socket_address
    );
    // create the stop handle container
    let stop_handle = web::Data::new(StopHandle::default());

    let server = HttpServer::new(move || {
        let stop_handle = stop_handle.clone();
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(stop_handle)
            .app_data(web::Data::new(network_spec.clone()))
            .app_data(web::Data::new(db.clone()))
            .configure(register_routers)
    })
    .bind(server_config.http_socket_address)?
    .run();

    server.await
}

#[derive(Default)]
struct StopHandle {
    inner: parking_lot::Mutex<Option<ServerHandle>>,
}

#[allow(dead_code)]
impl StopHandle {
    /// Sets the server handle to stop.
    pub(crate) fn register(&self, handle: ServerHandle) {
        *self.inner.lock() = Some(handle);
    }

    /// Sends stop signal through contained server handle.
    pub(crate) fn stop(&self, graceful: bool) {
        #[allow(clippy::let_underscore_future)]
        let _ = self.inner.lock().as_ref().unwrap().stop(graceful);
    }
}
