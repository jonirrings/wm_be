use crate::common::{BatchDelResult, ListingSpec};
use crate::databases::database::{Database, Error, Listing};
use crate::errors::ServiceError;
use crate::models::room::RoomId;
use crate::models::shelf::{Shelf, ShelfId};
use std::sync::Arc;

pub struct Service {
    shelf_repository: Arc<DbShelfRepository>,
}

impl Service {
    #[must_use]
    pub fn new(shelf_repository: Arc<DbShelfRepository>) -> Self {
        Self { shelf_repository }
    }
    pub async fn add_shelf(&self, name: &str, layer: i64, room_id: RoomId) -> Result<ShelfId, ServiceError> {
        self.shelf_repository
            .add(name, layer, room_id)
            .await
            .map_err(|_| ServiceError::InternalServerError)
    }
    pub async fn remove_shelf(&self, shelf_id: &ShelfId) -> Result<(), ServiceError> {
        self.shelf_repository
            .delete_one(&shelf_id)
            .await
            .map_err(|error: Error| match error {
                Error::ShelfNotFound => ServiceError::ShelfNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn remove_shelves(&self, ids: &Vec<ShelfId>) -> Result<BatchDelResult, ServiceError> {
        self.shelf_repository
            .delete_many(ids)
            .await
            .map_err(|error: Error| match error {
                Error::ShelfNotFound => ServiceError::ShelfNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn update_shelf(&self, shelf_id: &ShelfId, name: &str, layer: i64, room_id: RoomId) -> Result<(), ServiceError> {
        self.shelf_repository
            .update(&shelf_id, &name, layer, room_id)
            .await
            .map_err(|error: Error| match error {
                Error::ShelfNotFound => ServiceError::ShelfNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn update_shelf_name(&self, shelf_id: &ShelfId, name: &str) -> Result<(), ServiceError> {
        self.shelf_repository
            .update_name(&shelf_id, &name)
            .await
            .map_err(|error: Error| match error {
                Error::ShelfNotFound => ServiceError::ShelfNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn update_shelf_layer(&self, shelf_id: &ShelfId, layer: i64) -> Result<(), ServiceError> {
        self.shelf_repository
            .update_layer(&shelf_id, layer)
            .await
            .map_err(|error: Error| match error {
                Error::ShelfNotFound => ServiceError::ShelfNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn update_shelf_room(&self, shelf_id: &ShelfId, room_id: RoomId) -> Result<(), ServiceError> {
        self.shelf_repository
            .update_room(&shelf_id, room_id)
            .await
            .map_err(|error: Error| match error {
                Error::ShelfNotFound => ServiceError::ShelfNotFound,
                Error::RoomNotFound => ServiceError::RoomNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn get_shelf(&self, shelf_id: &ShelfId) -> Result<Shelf, ServiceError> {
        self.shelf_repository
            .get_one(shelf_id)
            .await
            .map_err(|_| ServiceError::ShelfNotFound)
    }
    pub async fn get_shelves(&self, spec: &ListingSpec, room_id: Option<RoomId>) -> Result<Listing<Shelf>, ServiceError> {
        self.shelf_repository
            .get_many(spec, room_id)
            .await
            .map_err(|_| ServiceError::InternalServerError)
    }
    pub async fn get_all_shelves(&self, room_id: Option<RoomId>) -> Result<Vec<Shelf>, ServiceError> {
        self.shelf_repository
            .get_all(room_id)
            .await
            .map_err(|_| ServiceError::InternalServerError)
    }
}

pub struct DbShelfRepository {
    database: Arc<Box<dyn Database>>,
}

impl DbShelfRepository {
    #[must_use]
    pub fn new(database: Arc<Box<dyn Database>>) -> Self {
        Self { database }
    }
    pub async fn add(&self, name: &str, layer: i64, room_id: RoomId) -> Result<ShelfId, Error> {
        self.database.insert_shelf_and_get_id(name, layer, room_id).await
    }
    pub async fn delete_one(&self, shelf_id: &ShelfId) -> Result<(), Error> {
        self.database.delete_shelf(*shelf_id).await
    }
    pub async fn delete_many(&self, ids: &Vec<ShelfId>) -> Result<BatchDelResult, Error> {
        self.database.delete_shelves(ids).await
    }
    pub async fn update(&self, shelf_id: &ShelfId, name: &str, layer: i64, room_id: RoomId) -> Result<(), Error> {
        self.database.update_shelf(*shelf_id, name, layer, room_id).await
    }
    pub async fn update_name(&self, shelf_id: &ShelfId, name: &str) -> Result<(), Error> {
        self.database.update_shelf_name(*shelf_id, name).await
    }
    pub async fn update_layer(&self, shelf_id: &ShelfId, layer: i64) -> Result<(), Error> {
        self.database.update_shelf_layer(*shelf_id, layer).await
    }
    pub async fn update_room(&self, shelf_id: &ShelfId, room_id: RoomId) -> Result<(), Error> {
        self.database.update_shelf_room(*shelf_id, room_id).await
    }
    pub async fn get_one(&self, shelf_id: &ShelfId) -> Result<Shelf, Error> {
        self.database.get_shelf_from_id(*shelf_id).await
    }
    pub async fn get_many(&self, spec: &ListingSpec, room_id: Option<RoomId>) -> Result<Listing<Shelf>, Error> {
        if let Some(room_id) = room_id {
            return self
                .database
                .get_shelves_in_room(spec.offset, spec.limit, &spec.sort, room_id)
                .await;
        }
        self.database.get_shelves(spec.offset, spec.limit, &spec.sort).await
    }
    pub async fn get_all(&self, room_id: Option<RoomId>) -> Result<Vec<Shelf>, Error> {
        if let Some(room_id) = room_id {
            return self.database.get_all_shelves_in_room(room_id).await;
        }
        self.database.get_all_shelves().await
    }
}
