use crate::config::Configuration;
use crate::databases::database::{Database, Sorting};
use crate::mailer;
use crate::models::room::RoomId;
use crate::models::shelf::ShelfId;
use crate::services::authentication::{DbUserAuthenticationRepository, JsonWebToken, Service};
use crate::services::item;
use crate::services::room;
use crate::services::shelf;
use crate::services::stock;
use crate::services::user::{self, DbBannedUserList, DbUserProfileRepository, DbUserRepository};
use crate::web::api::v1::auth::Authentication;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

pub struct AppData {
    pub cfg: Arc<Configuration>,
    pub database: Arc<Box<dyn Database>>,
    pub json_web_token: Arc<JsonWebToken>,
    pub auth: Arc<Authentication>,
    pub authentication_service: Arc<Service>,
    pub mailer: Arc<mailer::Service>,
    // Repositories
    pub user_repository: Arc<DbUserRepository>,
    pub user_authentication_repository: Arc<DbUserAuthenticationRepository>,
    pub user_profile_repository: Arc<DbUserProfileRepository>,
    pub banned_user_list: Arc<DbBannedUserList>,
    // Services
    pub registration_service: Arc<user::RegistrationService>,
    pub ban_service: Arc<user::BanService>,
    pub room_service: Arc<room::Service>,
    pub shelf_service: Arc<shelf::Service>,
    pub item_service: Arc<item::Service>,
    pub stock_service: Arc<stock::Service>,
}

impl AppData {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cfg: Arc<Configuration>,
        database: Arc<Box<dyn Database>>,
        json_web_token: Arc<JsonWebToken>,
        auth: Arc<Authentication>,
        authentication_service: Arc<Service>,
        mailer: Arc<mailer::Service>,
        // Repositories
        user_repository: Arc<DbUserRepository>,
        user_authentication_repository: Arc<DbUserAuthenticationRepository>,
        user_profile_repository: Arc<DbUserProfileRepository>,
        banned_user_list: Arc<DbBannedUserList>,
        // Services
        registration_service: Arc<user::RegistrationService>,
        ban_service: Arc<user::BanService>,
        room_service: Arc<room::Service>,
        shelf_service: Arc<shelf::Service>,
        item_service: Arc<item::Service>,
        stock_service: Arc<stock::Service>,
    ) -> Self {
        AppData {
            cfg,
            database,
            json_web_token,
            auth,
            authentication_service,
            mailer,
            // Repositories
            user_repository,
            user_authentication_repository,
            user_profile_repository,
            banned_user_list,
            // Services
            registration_service,
            ban_service,
            room_service,
            shelf_service,
            item_service,
            stock_service,
        }
    }
}

/// User request to generate a listing.
#[derive(Debug, Deserialize)]
pub struct ListingCriteria {
    pub offset: Option<u64>,
    pub limit: Option<u8>,
    pub sort: Option<Sorting>,
}
#[derive(Debug, Deserialize)]
pub struct PagedConf {
    pub all: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct FailureReason {
    name: String,
    reason: String,
}
#[derive(Debug, Serialize)]
pub struct BatchDelResult {
    pub s: u64,
    pub f: Option<Vec<FailureReason>>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Cond {
    Item,
    Shelf,
    Room,
}
#[derive(Debug, Deserialize)]
pub struct StockCond {
    pub cond: Cond,
}

#[derive(Debug, Deserialize)]
pub struct ExtraRoomId {
    pub room_id: Option<RoomId>,
}

#[derive(Debug, Deserialize)]
pub struct ExtraShelfId {
    pub shelf_id: Option<ShelfId>,
}

/// Internal specification for a listings.
#[derive(Debug, Deserialize)]
pub struct ListingSpec {
    pub offset: u64,
    pub limit: u8,
    pub sort: Sorting,
}
