use crate::common::ListingSpec;
use crate::databases::database::{Database, Error, Listing};
use crate::errors::ServiceError;
use crate::models::item::{Item, ItemId, ItemInRoom, ItemOnShelf, ItemXShelf};
use crate::models::room::RoomId;
use crate::models::shelf::ShelfId;
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
            .delete(item_id)
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
    pub async fn withdraw_item(&self, item_id: &ItemId, count: i64, shelf_id: ShelfId) -> Result<(), ServiceError> {
        self.item_repository
            .withdraw(item_id, count, shelf_id)
            .await
            .map_err(|error: Error| match error {
                Error::ItemNotFound => ServiceError::ItemNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn deposit_item(&self, item_id: &ItemId, count: i64, shelf_id: ShelfId) -> Result<(), ServiceError> {
        self.item_repository
            .deposit(item_id, count, shelf_id)
            .await
            .map_err(|error: Error| match error {
                Error::ItemNotFound => ServiceError::ItemNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn transfer_item(
        &self,
        item_id: &ItemId,
        count: i64,
        shelf_from: ShelfId,
        shelf_to: ShelfId,
    ) -> Result<(), ServiceError> {
        self.item_repository
            .transfer(item_id, count, shelf_from, shelf_to)
            .await
            .map_err(|error: Error| match error {
                Error::ItemNotFound => ServiceError::ItemNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn convert_item(&self, from: Vec<ItemXShelf>, into: Vec<ItemXShelf>) -> Result<(), ServiceError> {
        let len_from = from.len();
        if len_from == 0 {
            return Err(ServiceError::SourceMustBePositive);
        }
        let len_into = into.len();
        if len_into == 0 {
            return Err(ServiceError::TargetMustBePositive);
        }
        self.item_repository
            .convert(from, into)
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
    pub async fn get_items_on_shelf(&self, spec: &ListingSpec, shelf_id: ShelfId) -> Result<Listing<ItemOnShelf>, ServiceError> {
        self.item_repository
            .get_many_on_shelf(spec, shelf_id)
            .await
            .map_err(|_| ServiceError::InternalServerError)
    }
    pub async fn get_items_in_room(&self, spec: &ListingSpec, room_id: RoomId) -> Result<Listing<ItemInRoom>, ServiceError> {
        self.item_repository
            .get_many_in_room(spec, room_id)
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
    pub async fn delete(&self, item_id: &ItemId) -> Result<(), Error> {
        self.database.delete_item(*item_id).await
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
    pub async fn withdraw(&self, item_id: &ItemId, count: i64, shelf_id: ShelfId) -> Result<(), Error> {
        self.database.withdraw_items(*item_id, count, shelf_id).await
    }
    pub async fn deposit(&self, item_id: &ItemId, count: i64, shelf_id: ShelfId) -> Result<(), Error> {
        self.database.deposit_items(*item_id, count, shelf_id).await
    }
    pub async fn transfer(&self, item_id: &ItemId, count: i64, shelf_from: ShelfId, shelf_to: ShelfId) -> Result<(), Error> {
        self.database.transfer_items(*item_id, count, shelf_from, shelf_to).await
    }
    pub async fn convert(&self, from: Vec<ItemXShelf>, into: Vec<ItemXShelf>) -> Result<(), Error> {
        self.database.convert_items(from, into).await
    }
    pub async fn get_many(&self, spec: &ListingSpec) -> Result<Listing<Item>, Error> {
        self.database.get_items(spec.offset, spec.limit, &spec.sort).await
    }
    pub async fn get_many_on_shelf(&self, spec: &ListingSpec, shelf_id: ShelfId) -> Result<Listing<ItemOnShelf>, Error> {
        self.database
            .get_items_on_shelf(spec.offset, spec.limit, &spec.sort, shelf_id)
            .await
    }
    pub async fn get_many_in_room(&self, spec: &ListingSpec, room_id: RoomId) -> Result<Listing<ItemInRoom>, Error> {
        self.database
            .get_items_in_room(spec.offset, spec.limit, &spec.sort, room_id)
            .await
    }
}
