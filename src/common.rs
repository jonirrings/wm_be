use std::sync::Arc;
use serde_derive::Deserialize;
use crate::config::Configuration;
use crate::databases::database::{Database, Sorting};
use crate::services::authentication::{DbUserAuthenticationRepository, JsonWebToken, Service};
use crate::services::user::{self, DbBannedUserList, DbUserProfileRepository, DbUserRepository};
use crate::services::room::{self, DbRoomRepository};
use crate::services::shelf::{self, DbShelfRepository};
use crate::services::item::{self, DbItemRepository};
use crate::web::api::v1::auth::Authentication;
use crate::mailer;
use crate::models::room::RoomId;
use crate::models::shelf::ShelfId;

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
    pub room_repository: Arc<DbRoomRepository>,
    pub shelf_repository: Arc<DbShelfRepository>,
    // Services
    pub registration_service: Arc<user::RegistrationService>,
    pub ban_service: Arc<user::BanService>,
    pub room_service: Arc<room::Service>,
    pub shelf_service: Arc<shelf::Service>,
    pub item_service: Arc<item::Service>,
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
        room_repository: Arc<DbRoomRepository>,
        shelf_repository: Arc<DbShelfRepository>,
        // Services
        registration_service: Arc<user::RegistrationService>,
        ban_service: Arc<user::BanService>,
        room_service: Arc<room::Service>,
        shelf_service: Arc<shelf::Service>,
        item_service: Arc<item::Service>,
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
            room_repository,
            shelf_repository,
            // Services
            registration_service,
            ban_service,
            room_service,
            shelf_service,
            item_service
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
pub struct ExtraRoomId{
    pub room_id:Option<RoomId>
}
#[derive(Debug, Deserialize)]
pub struct ExtraShelfId{
    pub shelf_id:Option<ShelfId>
}

/// Internal specification for a listings.
#[derive(Debug, Deserialize)]
pub struct ListingSpec {
    pub offset: u64,
    pub limit: u8,
    pub sort: Sorting,
}
