use crate::common::{BatchDelResult, ListingSpec};
use crate::databases::database::{Database, Error, Listing};
use crate::errors::ServiceError;
use crate::models::item::{Item, ItemId};
use std::sync::Arc;

pub struct Service {
    item_repository: Arc<DbItemRepository>,
}

impl Service {
    #[must_use]
    pub fn new(item_repository: Arc<DbItemRepository>) -> Self {
        Self { item_repository }
    }
    pub async fn add_item(&self, name: &str, sn: &str) -> Result<ItemId, ServiceError> {
        self.item_repository
            .add(name, sn)
            .await
            .map_err(|_| ServiceError::InternalServerError)
    }
    pub async fn add_item_with_desc(&self, name: &str, desc: &str, sn: &str) -> Result<ItemId, ServiceError> {
        self.item_repository
            .add_with_desc(name, desc, sn)
            .await
            .map_err(|_| ServiceError::InternalServerError)
    }
    pub async fn remove_item(&self, item_id: &ItemId) -> Result<(), ServiceError> {
        self.item_repository
            .delete_one(item_id)
            .await
            .map_err(|error: Error| match error {
                Error::ItemNotFound => ServiceError::ItemNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn remove_items(&self, ids: &Vec<ItemId>) -> Result<BatchDelResult, ServiceError> {
        self.item_repository
            .delete_many(ids)
            .await
            .map_err(|error: Error| match error {
                Error::ItemNotFound => ServiceError::ItemNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn update_item(&self, item_id: &ItemId, name: &str, desc: &Option<String>, sn: &str) -> Result<(), ServiceError> {
        self.item_repository
            .update(item_id, name, desc, sn)
            .await
            .map_err(|error: Error| match error {
                Error::ItemNotFound => ServiceError::ItemNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn update_item_name(&self, item_id: &ItemId, name: &str) -> Result<(), ServiceError> {
        self.item_repository
            .update_name(item_id, name)
            .await
            .map_err(|error: Error| match error {
                Error::ItemNotFound => ServiceError::ItemNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn update_item_desc(&self, item_id: &ItemId, desc: &str) -> Result<(), ServiceError> {
        self.item_repository
            .update_desc(item_id, desc)
            .await
            .map_err(|error: Error| match error {
                Error::ItemNotFound => ServiceError::ItemNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn update_item_sn(&self, item_id: &ItemId, sn: &str) -> Result<(), ServiceError> {
        self.item_repository
            .update_sn(item_id, sn)
            .await
            .map_err(|error: Error| match error {
                Error::ItemNotFound => ServiceError::ItemNotFound,
                _ => ServiceError::InternalServerError,
            })
    }

    pub async fn get_item(&self, item_id: &ItemId) -> Result<Item, ServiceError> {
        self.item_repository
            .get_one(item_id)
            .await
            .map_err(|_| ServiceError::ItemNotFound)
    }
    pub async fn get_items(&self, spec: &ListingSpec) -> Result<Listing<Item>, ServiceError> {
        self.item_repository
            .get_many(spec)
            .await
            .map_err(|_| ServiceError::InternalServerError)
    }
    pub async fn get_all_items(&self) -> Result<Vec<Item>, ServiceError> {
        self.item_repository
            .get_all()
            .await
            .map_err(|_| ServiceError::InternalServerError)
    }
}

pub struct DbItemRepository {
    database: Arc<Box<dyn Database>>,
}

impl DbItemRepository {
    #[must_use]
    pub fn new(database: Arc<Box<dyn Database>>) -> Self {
        Self { database }
    }
    pub async fn add(&self, name: &str, sn: &str) -> Result<ItemId, Error> {
        self.database.insert_item_and_get_id(name, sn).await
    }
    pub async fn add_with_desc(&self, name: &str, desc: &str, sn: &str) -> Result<ItemId, Error> {
        self.database.insert_item_with_desc_and_get_id(name, desc, sn).await
    }
    pub async fn delete_one(&self, item_id: &ItemId) -> Result<(), Error> {
        self.database.delete_item(*item_id).await
    }
    pub async fn delete_many(&self, item_ids: &Vec<ItemId>) -> Result<BatchDelResult, Error> {
        self.database.delete_items(item_ids).await
    }
    pub async fn update(&self, item_id: &ItemId, name: &str, desc: &Option<String>, sn: &str) -> Result<(), Error> {
        self.database.update_item(*item_id, name, desc, sn).await
    }
    pub async fn update_name(&self, item_id: &ItemId, name: &str) -> Result<(), Error> {
        self.database.update_item_name(*item_id, name).await
    }
    pub async fn update_desc(&self, item_id: &ItemId, desc: &str) -> Result<(), Error> {
        self.database.update_item_desc(*item_id, desc).await
    }
    pub async fn update_sn(&self, item_id: &ItemId, sn: &str) -> Result<(), Error> {
        self.database.update_item_sn(*item_id, sn).await
    }
    pub async fn get_one(&self, item_id: &ItemId) -> Result<Item, Error> {
        self.database.get_item_from_id(*item_id).await
    }
    pub async fn get_many(&self, spec: &ListingSpec) -> Result<Listing<Item>, Error> {
        self.database.get_items(spec.offset, spec.limit, &spec.sort).await
    }
    pub async fn get_all(&self) -> Result<Vec<Item>, Error> {
        self.database.get_all_items().await
    }
}
