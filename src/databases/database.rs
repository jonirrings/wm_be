use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// use crate::databases::mysql::Mysql;
use crate::databases::sqlite::Sqlite;
use crate::models::item::{Item, ItemId, ItemInRoom, ItemOnShelf, ItemXShelf};
use crate::models::room::{Room, RoomId};
use crate::models::shelf::{Shelf, ShelfId};
// use crate::databases::postgres::Postgres;
use crate::models::user::{User, UserAuthentication, UserCompact, UserId, UserProfile};

/// Database drivers.
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum Driver {
    Sqlite3,
    // Mysql,
    // Postgres,
}

/// Sorting options for Item.
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Sorting {
    NameAsc,
    NameDesc,
    IdAsc,
    IdDesc,
}

/// Database errors.
#[derive(Debug)]
pub enum Error {
    Error,
    ErrorWithText(String),
    UnrecognizedDatabaseDriver,
    // when the db path does not start with sqlite or mysql
    UsernameTaken,
    EmailTaken,
    UserNotFound,
    RoomNotFound,
    ShelfNotFound,
    ItemNotFound,
    CountMustBePositive,
    InsufficientItem,
}

/// Get the Driver of the Database from the Connection String
///
/// # Errors
///
/// This function will return an `Error::UnrecognizedDatabaseDriver` if unable to match database type.
pub fn get_driver(db_path: &str) -> Result<Driver, Error> {
    match &db_path.chars().collect::<Vec<char>>() as &[char] {
        ['s', 'q', 'l', 'i', 't', 'e', ..] => Ok(Driver::Sqlite3),
        // ['m', 'y', 's', 'q', 'l', ..] => Ok(Driver::Mysql),
        // ['p', 'o', 's', 't', 'g', 'r', 'e', 's', 'q', 'l', ..] => Ok(Driver::Postgres),
        _ => Err(Error::UnrecognizedDatabaseDriver),
    }
}

/// Connect to a database.
///
/// # Errors
///
/// This function will return an `Error::UnrecognizedDatabaseDriver` if unable to match database type.
pub async fn connect(db_path: &str) -> Result<Box<dyn Database>, Error> {
    let db_driver = self::get_driver(db_path)?;

    Ok(match db_driver {
        Driver::Sqlite3 => Box::new(Sqlite::new(db_path).await),
        // Driver::Mysql => Box::new(Mysql::new(db_path).await),
        // Driver::Postgres => Box::new(Postgres::new(db_path).await),
    })
}

/// Trait for database implementations.
#[async_trait]
pub trait Database: Sync + Send {
    /// Return current database driver.
    fn get_database_driver(&self) -> Driver;
    async fn new(db_path: &str) -> Self where Self: Sized;

    /// Add new user and return the newly inserted `user_id`.
    async fn insert_user_and_get_id(&self, username: &str, email: &str, password: &str) -> Result<UserId, Error>;
    /// Get `User` from `user_id`.
    async fn get_user_from_id(&self, user_id: i64) -> Result<User, Error>;

    /// Get `UserAuthentication` from `user_id`.
    async fn get_user_authentication_from_id(&self, user_id: UserId) -> Result<UserAuthentication, Error>;

    /// Get `UserProfile` from `username`.
    async fn get_user_profile_from_username(&self, username: &str) -> Result<UserProfile, Error>;
    /// Get `UserProfile` from `user_id`.
    async fn get_user_profile_from_id(&self, user_id: UserId) -> Result<UserProfile, Error>;

    /// Get `UserCompact` from `user_id`.
    async fn get_user_compact_from_id(&self, user_id: UserId) -> Result<UserCompact, Error>;
    /// Get total user count.
    async fn count_users(&self) -> Result<i64, Error>;

    /// Grant a user the administrator role.
    async fn grant_admin_role(&self, user_id: UserId) -> Result<(), Error>;

    /// Ban user with `user_id`, `reason` and `date_expiry`.
    async fn ban_user(&self, user_id: UserId, reason: &str, date_expiry: NaiveDateTime) -> Result<(), Error>;
    /// Verify a user's email with `user_id`.
    async fn verify_email(&self, user_id: UserId) -> Result<(), Error>;
    /// Delete user and all related user data with `user_id`.
    async fn delete_user(&self, user_id: UserId) -> Result<(), Error>;
    /// Add a new room
    async fn insert_room_and_get_id(&self, name: &str) -> Result<i64, Error>;
    async fn insert_room_with_desc_and_get_id(&self, name: &str, desc: &str) -> Result<i64, Error>;
    /// Delete a room.
    async fn delete_room(&self, room_id: RoomId) -> Result<(), Error>;
    /// Update a room's name with `room_id`.
    async fn update_room_name(&self, room_id: RoomId, name: &str) -> Result<(), Error>;
    /// Update a room's description with `room_id`.
    async fn update_room_desc(&self, room_id: RoomId, desc: &str) -> Result<(), Error>;
    /// Get a `room` from `room_id`.
    async fn get_room_from_id(&self, room_id: RoomId) -> Result<Room, Error>;
    /// Get 'rooms' from criteria
    async fn get_rooms(&self, offset: u64, limit: u8, sort: &Sorting) -> Result<Listing<Room>, Error>;
    async fn insert_shelf_and_get_id(&self, name: &str, layer: i64, room_id: RoomId) -> Result<ShelfId, Error>;
    async fn delete_shelf(&self, shelf_id: ShelfId) -> Result<(), Error>;
    async fn update_shelf_name(&self, shelf_id: ShelfId, name: &str) -> Result<(), Error>;
    async fn update_shelf_layer(&self, shelf_id: ShelfId, layer: i64) -> Result<(), Error>;
    async fn update_shelf_room(&self, shelf_id: ShelfId, room_id: RoomId) -> Result<(), Error>;
    async fn get_shelf_from_id(&self, shelf_id: ShelfId) -> Result<Shelf, Error>;
    async fn get_shelves(&self, offset: u64, limit: u8, sort: &Sorting) -> Result<Listing<Shelf>, Error>;
    async fn get_shelves_in_room(&self, offset: u64, limit: u8, sort: &Sorting, room_id: RoomId) -> Result<Listing<Shelf>, Error>;
    async fn insert_item_and_get_id(&self, name: &str, sn: &str) -> Result<ItemId, Error>;
    async fn insert_item_with_desc_and_get_id(&self, name: &str, desc: &str, sn: &str) -> Result<ItemId, Error>;
    async fn delete_item(&self, item_id: ItemId) -> Result<(), Error>;
    async fn update_item_name(&self, item_id: ItemId, name: &str) -> Result<(), Error>;
    async fn update_item_desc(&self, item_id: ItemId, desc: &str) -> Result<(), Error>;
    async fn update_item_sn(&self, item_id: ItemId, sn: &str) -> Result<(), Error>;
    async fn get_item_from_id(&self, item_id: ItemId) -> Result<Item, Error>;
    async fn get_items(&self, offset: u64, limit: u8, sort: &Sorting) -> Result<Listing<Item>, Error>;
    async fn get_items_on_shelf(&self, offset: u64, limit: u8, sort: &Sorting, shelf_id: ShelfId) -> Result<Listing<ItemOnShelf>, Error>;
    async fn get_items_in_room(&self, offset: u64, limit: u8, sort: &Sorting, room_id: RoomId) -> Result<Listing<ItemInRoom>, Error>;
    async fn transfer_items(&self, item_id: ItemId, count: i64, shelf_from: ShelfId, shelf_to: ShelfId) -> Result<(), Error>;
    async fn withdraw_items(&self, item_id: ItemId, count: i64, shelf_id: ShelfId) -> Result<(), Error>;
    async fn deposit_items(&self, item_id: ItemId, count: i64, shelf_id: ShelfId) -> Result<(), Error>;
    async fn convert_items(&self, from: Vec<ItemXShelf>, into: Vec<ItemXShelf>, ) -> Result<(), Error>;
}
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Listing<T>{
    pub total: u64,
    pub data: Vec<T>,
}
