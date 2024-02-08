use std::sync::Arc;

use crate::common::{BatchDelResult, ListingSpec};
use crate::databases::database::{Database, Error, Listing};
use crate::errors::ServiceError;
use crate::models::room::{Room, RoomId};
use crate::web::api::v1::contexts::room::forms::AddRoomForm;

pub struct Service {
    room_repository: Arc<DbRoomRepository>,
}

impl Service {
    #[must_use]
    pub fn new(room_repository: Arc<DbRoomRepository>) -> Self {
        Self { room_repository }
    }
    pub async fn add_room(
        &self,
        registration_form: &AddRoomForm, /*, opt_user_id: Option<UserId>*/
    ) -> Result<RoomId, ServiceError> {
        if let Some(desc) = &registration_form.description {
            if desc.len() > 200 {
                return Err(ServiceError::DescNotValid);
            }
            return self
                .room_repository
                .add_with_desc(&registration_form.name, desc)
                .await
                .map_err(|_| ServiceError::InternalServerError);
        }
        self.room_repository
            .add(&registration_form.name)
            .await
            .map_err(|_| ServiceError::InternalServerError)
    }
    pub async fn close_room(&self, room_id: &RoomId) -> Result<(), ServiceError> {
        self.room_repository
            .delete_one(&room_id)
            .await
            .map_err(|error: Error| match error {
                Error::RoomNotFound => ServiceError::RoomNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn close_rooms(&self, ids: &Vec<RoomId>) -> Result<BatchDelResult, ServiceError> {
        self.room_repository
            .delete_many(ids)
            .await
            .map_err(|error: Error| match error {
                Error::RoomNotFound => ServiceError::RoomNotFound,
                _ => ServiceError::InternalServerError,
            })
    }

    pub async fn update_room(&self, room_id: &RoomId, name: &str, desc: &Option<String>) -> Result<(), ServiceError> {
        self.room_repository
            .update(&room_id, name, desc)
            .await
            .map_err(|error: Error| match error {
                Error::RoomNotFound => ServiceError::RoomNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn update_room_name(&self, room_id: &RoomId, name: &str) -> Result<(), ServiceError> {
        self.room_repository
            .update_name(&room_id, &name)
            .await
            .map_err(|error: Error| match error {
                Error::RoomNotFound => ServiceError::RoomNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn update_room_desc(&self, room_id: &RoomId, desc: &str) -> Result<(), ServiceError> {
        self.room_repository
            .update_name(&room_id, &desc)
            .await
            .map_err(|error: Error| match error {
                Error::RoomNotFound => ServiceError::RoomNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn get_room(&self, room_id: &RoomId /*, opt_user_id: Option<UserId>*/) -> Result<Room, ServiceError> {
        self.room_repository
            .get_one(room_id)
            .await
            .map_err(|_| ServiceError::RoomNotFound)
    }
    pub async fn get_rooms(&self, spec: &ListingSpec) -> Result<Listing<Room>, ServiceError> {
        self.room_repository
            .get_many(&spec)
            .await
            .map_err(|_| ServiceError::InternalServerError)
    }
    pub async fn get_all_rooms(&self) -> Result<Vec<Room>, ServiceError> {
        self.room_repository
            .get_all()
            .await
            .map_err(|_| ServiceError::InternalServerError)
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
    pub async fn delete_one(&self, room_id: &RoomId) -> Result<(), Error> {
        self.database.delete_room(*room_id).await
    }
    pub async fn delete_many(&self, ids: &Vec<RoomId>) -> Result<BatchDelResult, Error> {
        self.database.delete_rooms(ids).await
    }
    pub async fn update(&self, room_id: &RoomId, name: &str, desc: &Option<String>) -> Result<(), Error> {
        self.database.update_room(*room_id, name, desc).await
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
    pub async fn get_many(&self, spec: &ListingSpec) -> Result<Listing<Room>, Error> {
        self.database.get_rooms(spec.offset, spec.limit, &spec.sort).await
    }
    pub async fn get_all(&self) -> Result<Vec<Room>, Error> {
        self.database.get_all_rooms().await
    }
}
