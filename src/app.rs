use std::net::SocketAddr;
use std::sync::Arc;
use tokio::task::JoinHandle;

use crate::bootstrap::logging;
use crate::common::AppData;
use crate::config::Configuration;
use crate::databases::database;
use crate::web;
use crate::web::api::Version;

pub struct Running {
    pub api_socket_addr: SocketAddr,
    pub api_server: Option<JoinHandle<std::result::Result<(), std::io::Error>>>,
}
#[allow(clippy::too_many_lines)]
pub async fn run(configuration: Configuration, api_version: &Version) -> Running {
    let log_level = configuration.settings.read().await.log_level.clone();
    logging::setup(&log_level);

    let configuration = Arc::new(configuration);

    // Get configuration settings needed to build the app dependencies and
    // services: main API server and tracker torrents importer.

    let settings = configuration.settings.read().await;

    // From [database] config
    let database_connect_url = settings.database.connect_url.clone();
    // From [net] config
    let net_ip = "127.0.0.1".to_string();
    let net_port = settings.net.port;
    // IMPORTANT: drop settings before starting server to avoid read locks that
    // leads to requests hanging.
    drop(settings);
    // Build app dependencies

    let database = Arc::new(database::connect(&database_connect_url).await.expect("Database error."));
    // Build app container
    let app_data = Arc::new(AppData::new(configuration.clone(),
                                         database.clone(),));
    // Start API server
    let running_api = web::api::start(app_data, &net_ip, net_port, api_version).await;
    Running {
        api_socket_addr: running_api.socket_addr,
        api_server: running_api.api_server,
    }
}