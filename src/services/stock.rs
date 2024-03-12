use std::sync::Arc;

use crate::common::ListingSpec;
use crate::databases::database::{Database, Error, Listing};
use crate::errors::ServiceError;
use crate::models::item::{ItemId, ItemInRoom, ItemOnShelf, ItemXShelf};
use crate::models::room::RoomId;
use crate::models::shelf::ShelfId;

pub struct Service {
    stock_repository: Arc<DbStockRepository>,
}

impl Service {
    #[must_use]
    pub fn new(stock_repository: Arc<DbStockRepository>) -> Self {
        Self { stock_repository }
    }
    pub async fn withdraw_item(&self, item_id: &ItemId, count: i64, shelf_id: ShelfId) -> Result<(), ServiceError> {
        self.stock_repository
            .withdraw(item_id, count, shelf_id)
            .await
            .map_err(|error: Error| match error {
                Error::ItemNotFound => ServiceError::ItemNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn deposit_item(&self, item_id: &ItemId, count: i64, shelf_id: ShelfId) -> Result<(), ServiceError> {
        self.stock_repository
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
        self.stock_repository
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
        self.stock_repository
            .convert(from, into)
            .await
            .map_err(|error: Error| match error {
                Error::ItemNotFound => ServiceError::ItemNotFound,
                _ => ServiceError::InternalServerError,
            })
    }
    pub async fn get_items_on_shelf(&self, spec: &ListingSpec, shelf_id: ShelfId) -> Result<Listing<ItemOnShelf>, ServiceError> {
        self.stock_repository
            .get_many_on_shelf(spec, shelf_id)
            .await
            .map_err(|_| ServiceError::InternalServerError)
    }
    pub async fn get_items_in_room(&self, spec: &ListingSpec, room_id: RoomId) -> Result<Listing<ItemInRoom>, ServiceError> {
        self.stock_repository
            .get_many_in_room(spec, room_id)
            .await
            .map_err(|_| ServiceError::InternalServerError)
    }
}

pub struct DbStockRepository {
    database: Arc<Box<dyn Database>>,
}

impl DbStockRepository {
    #[must_use]
    pub fn new(database: Arc<Box<dyn Database>>) -> Self {
        Self { database }
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
    pub async fn get_many_on_shelves(&self, spec: &ListingSpec) -> Result<Listing<ItemOnShelf>, Error> {
        self.database.get_stocks_on_shelves(spec.offset, spec.limit, &spec.sort).await
    }
    pub async fn get_many_on_shelf(&self, spec: &ListingSpec, shelf_id: ShelfId) -> Result<Listing<ItemOnShelf>, Error> {
        self.database
            .get_stocks_on_shelf(spec.offset, spec.limit, &spec.sort, shelf_id)
            .await
    }
    pub async fn get_many_in_rooms(&self, spec: &ListingSpec) -> Result<Listing<ItemInRoom>, Error> {
        self.database.get_stocks_in_rooms(spec.offset, spec.limit, &spec.sort).await
    }
    pub async fn get_many_in_room(&self, spec: &ListingSpec, room_id: RoomId) -> Result<Listing<ItemInRoom>, Error> {
        self.database
            .get_stocks_in_room(spec.offset, spec.limit, &spec.sort, room_id)
            .await
    }
}
