use std::sync::Arc;

use serde_derive::{Deserialize, Serialize};

use crate::config::Configuration;
use crate::errors::ServiceError;
use crate::databases::database::{Database, Error, Rooms, Sorting};
use crate::models::room::{Room, RoomId};
use crate::models::user::UserId;
use crate::web::api::v1::contexts::room::forms::AddRoomForm;

pub struct Service {
    configuration: Arc<Configuration>,
    room_repository: Arc<DbRoomRepository>,
}

/// User request to generate a listing.
#[derive(Debug, Deserialize)]
pub struct ListingRequest {
    pub offset: Option<u64>,
    pub limit: Option<u8>,
    pub sort: Option<Sorting>,
}

/// Internal specification for a listings.
#[derive(Debug, Deserialize)]
pub struct ListingSpecification {
    pub offset: u64,
    pub limit: u8,
    pub sort: Sorting,
}

impl Service {
    #[must_use]
    pub fn new(
        configuration: Arc<Configuration>, room_repository: Arc<DbRoomRepository>) -> Self {
        Self { configuration, room_repository }
    }
    pub async fn add_room(&self, registration_form: &AddRoomForm/*, opt_user_id: Option<UserId>*/) -> Result<RoomId, ServiceError> {
        //fixme: check the user permission
        /*if opt_user_id.is_none() {
            return Err(ServiceError::Unauthorized)
        }*/
        if let Some(desc) = &registration_form.description {
            if desc.len() > 200 {
                return Err(ServiceError::RoomDescNotValid);
            }
            return self.room_repository.add_with_desc(&registration_form.name, desc).await.map_err(|_| ServiceError::InternalServerError);
        }
        self.room_repository.add(&registration_form.name).await.map_err(|_| ServiceError::InternalServerError)
    }
    pub async fn close_room(&self, room_id: &RoomId) -> Result<(), ServiceError> {
        self.room_repository.delete(&room_id).await.map_err(|error: Error| match error {
            Error::RoomNotFound => ServiceError::RoomNotFound,
            _ => ServiceError::InternalServerError
        })
    }
    pub async fn update_room_name(&self, room_id: &RoomId, name: &str) -> Result<(), ServiceError> {
        self.room_repository.update_name(&room_id, &name).await.map_err(|error: Error| match error {
            Error::RoomNotFound => ServiceError::RoomNotFound,
            _ => ServiceError::InternalServerError
        })
    }
    pub async fn update_room_desc(&self, room_id: &RoomId, desc: &str) -> Result<(), ServiceError> {
        self.room_repository.update_name(&room_id, &desc).await.map_err(|error: Error| match error {
            Error::RoomNotFound => ServiceError::RoomNotFound,
            _ => ServiceError::InternalServerError
        })
    }
    pub async fn get_room(&self, room_id: &RoomId/*, opt_user_id: Option<UserId>*/) -> Result<Room, ServiceError> {
        //fixme: check the user permission
        /*if opt_user_id.is_none() {
            return Err(ServiceError::Unauthorized)
        }*/
        self.room_repository.get_one(room_id).await.map_err(|_| ServiceError::RoomNotFound)
    }
    pub async fn get_rooms(&self, request: &ListingRequest) -> Result<Rooms, ServiceError> {
        //fixme: check the user permission
        /*if opt_user_id.is_none() {
            return Err(ServiceError::Unauthorized)
        }*/
        let spec = self.spec_from_request(request).await;
        self.room_repository.get_many(&spec).await.map_err(|_| ServiceError::RoomNotFound)
    }
    async fn spec_from_request(&self, request: &ListingRequest) -> ListingSpecification {
        let settings = self.configuration.settings.read().await;
        let default_page_size = settings.api.default_page_size;
        let max_page_size = settings.api.max_page_size;
        drop(settings);
        let sort = request.sort.unwrap_or(Sorting::IdAsc);
        let offset = request.offset.unwrap_or(0);
        let limit = request.limit.unwrap_or(default_page_size);
        let limit = if limit > max_page_size {
            max_page_size
        } else {
            limit
        };
        ListingSpecification {
            offset,
            limit,
            sort,
        }
    }
}

pub struct DbRoomRepository {
    database: Arc<Box<dyn Database>>,
}

impl DbRoomRepository {
    #[must_use]
    pub fn new(database: Arc<Box<dyn Database>>) -> Self {
        Self { database }
    }
    pub async fn add(&self, name: &str) -> Result<RoomId, Error> {
        self.database.insert_room_and_get_id(name).await
    }
    pub async fn add_with_desc(&self, name: &str, desc: &str) -> Result<RoomId, Error> {
        self.database.insert_room_with_desc_and_get_id(name, desc).await
    }
    pub async fn delete(&self, room_id: &RoomId) -> Result<(), Error> {
        self.database.delete_room(*room_id).await
    }
    pub async fn update_name(&self, room_id: &RoomId, name: &str) -> Result<(), Error> {
        self.database.update_room_name(*room_id, name).await
    }
    pub async fn update_desc(&self, room_id: &RoomId, desc: &str) -> Result<(), Error> {
        self.database.update_room_desc(*room_id, desc).await
    }
    pub async fn get_one(&self, room_id: &RoomId) -> Result<Room, Error> {
        self.database.get_room_from_id(*room_id).await
    }
    pub async fn get_many(&self, spec: &ListingSpecification) -> Result<Rooms, Error> {
        self.database.get_rooms(spec.offset, spec.limit, &spec.sort).await
    }
}