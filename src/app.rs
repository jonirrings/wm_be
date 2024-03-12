use std::net::SocketAddr;
use std::sync::Arc;

use tokio::task::JoinHandle;

use crate::bootstrap::logging;
use crate::common::AppData;
use crate::config::Configuration;
use crate::databases::database;
use crate::services::authentication::{DbUserAuthenticationRepository, JsonWebToken, Service};
use crate::services::item::{self, DbItemRepository};
use crate::services::room::{self, DbRoomRepository};
use crate::services::shelf::{self, DbShelfRepository};
use crate::services::stock::{self, DbStockRepository};
use crate::services::user::{self, DbBannedUserList, DbUserProfileRepository, DbUserRepository};
use crate::web::api::v1::auth::Authentication;
use crate::web::api::Version;
use crate::{mailer, web};

pub struct Running {
    pub api_socket_addr: SocketAddr,
    pub api_server: Option<JoinHandle<Result<(), std::io::Error>>>,
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
    let net_ip = settings.net.v4.clone().unwrap_or("localhost".to_string());
    let net_port = settings.net.v4port;
    // IMPORTANT: drop settings before starting server to avoid read locks that
    // leads to requests hanging.
    drop(settings);
    // Build app dependencies

    let database = Arc::new(database::connect(&database_connect_url).await.expect("Database error."));
    let json_web_token = Arc::new(JsonWebToken::new(configuration.clone()));
    let auth = Arc::new(Authentication::new(json_web_token.clone()));
    // Repositories
    let user_repository = Arc::new(DbUserRepository::new(database.clone()));
    let user_authentication_repository = Arc::new(DbUserAuthenticationRepository::new(database.clone()));
    let user_profile_repository = Arc::new(DbUserProfileRepository::new(database.clone()));
    let banned_user_list = Arc::new(DbBannedUserList::new(database.clone()));
    let room_repository = Arc::new(DbRoomRepository::new(database.clone()));
    let shelf_repository = Arc::new(DbShelfRepository::new(database.clone()));
    let item_repository = Arc::new(DbItemRepository::new(database.clone()));
    let stock_repository = Arc::new(DbStockRepository::new(database.clone()));
    // Services
    let mailer_service = Arc::new(mailer::Service::new(configuration.clone()).await);
    let registration_service = Arc::new(user::RegistrationService::new(
        configuration.clone(),
        mailer_service.clone(),
        user_repository.clone(),
        user_profile_repository.clone(),
    ));
    let ban_service = Arc::new(user::BanService::new(
        user_repository.clone(),
        user_profile_repository.clone(),
        banned_user_list.clone(),
    ));
    let room_service = Arc::new(room::Service::new(room_repository.clone()));
    let shelf_service = Arc::new(shelf::Service::new(shelf_repository.clone()));
    let item_service = Arc::new(item::Service::new(item_repository.clone()));
    let stock_service = Arc::new(stock::Service::new(stock_repository.clone()));
    let authentication_service = Arc::new(Service::new(
        configuration.clone(),
        json_web_token.clone(),
        user_repository.clone(),
        user_profile_repository.clone(),
        user_authentication_repository.clone(),
    ));
    // Build app container
    let app_data = Arc::new(AppData::new(
        configuration.clone(),
        database.clone(),
        json_web_token.clone(),
        auth.clone(),
        authentication_service,
        mailer_service,
        user_repository,
        user_authentication_repository,
        user_profile_repository,
        banned_user_list,
        registration_service,
        ban_service,
        room_service,
        shelf_service,
        item_service,
        stock_service,
    ));
    // Start API server
    let running_api = web::api::start(app_data, &net_ip, net_port, api_version).await;
    Running {
        api_socket_addr: running_api.socket_addr,
        api_server: running_api.api_server,
    }
}
