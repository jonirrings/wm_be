use std::str::FromStr;
use std::time::Duration;
use std::u64;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{query, query_as, Acquire, ConnectOptions, SqlitePool};

use crate::common::BatchDelResult;
use crate::databases::database::{self, Database, Driver, Error, Listing, Sorting};
use crate::models::item::{Item, ItemId, ItemInRoom, ItemOnShelf, ItemXShelf};
use crate::models::room::{Room, RoomId};
use crate::models::shelf::{Shelf, ShelfId};
use crate::models::user::{User, UserAuthentication, UserCompact, UserId, UserProfile};

pub struct Sqlite {
    pub pool: SqlitePool,
}

//fixme foreign key error
#[async_trait]
impl Database for Sqlite {
    fn get_database_driver(&self) -> Driver {
        Driver::Sqlite3
    }
    async fn new(database_url: &str) -> Self {
        let connection_options = SqliteConnectOptions::from_str(database_url)
            .expect("Unable to create connection options.")
            .log_statements(log::LevelFilter::Debug)
            .log_slow_statements(log::LevelFilter::Info, Duration::from_secs(1));

        let db = SqlitePoolOptions::new()
            .connect_with(connection_options)
            .await
            .expect("Unable to create database pool.");

        sqlx::migrate!("migrations/sqlite3")
            .run(&db)
            .await
            .expect("Could not run database migrations.");

        Self { pool: db }
    }

    async fn insert_user_and_get_id(&self, username: &str, email: &str, password_hash: &str) -> Result<i64, Error> {
        // open pool connection
        let mut conn = self.pool.acquire().await.map_err(|_| Error::ConnectionPoolFailed)?;
        // start db transaction
        let mut tx = conn.begin().await.map_err(|_| Error::TransactionError)?;
        // create the user account and get the user id
        let sql = "INSERT INTO users (created_at) VALUES (datetime('now'))";
        let user_id = query(sql)
            .execute(&mut *tx)
            .await
            .map(|v| v.last_insert_rowid())
            .map_err(|_| Error::Error)?;
        // add password hash for account
        let sql = "INSERT INTO user_authentication (user_id, password_hash) VALUES (?, ?)";
        let insert_user_auth_result = query(sql)
            .bind(user_id)
            .bind(password_hash)
            .execute(&mut *tx)
            .await
            .map_err(|_| Error::Error);
        // rollback transaction on error
        if let Err(e) = insert_user_auth_result {
            drop(tx.rollback().await);
            return Err(e);
        }
        // add account profile details
        let sql = r#"INSERT INTO user_profiles (user_id, username, email, email_verified, bio, avatar) VALUES (?, ?, NULLIF(?, ""), 0, NULL, NULL)"#;
        let insert_user_profile_result = query(sql)
            .bind(user_id)
            .bind(username)
            .bind(email)
            .execute(&mut *tx)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(err) => {
                    if err.message().contains("username") {
                        Error::UsernameTaken
                    } else if err.message().contains("email") {
                        Error::EmailTaken
                    } else {
                        Error::Error
                    }
                }
                _ => Error::Error,
            });
        // commit or rollback transaction and return user_id on success
        match insert_user_profile_result {
            Ok(_) => {
                drop(tx.commit().await);
                Ok(user_id)
            }
            Err(e) => {
                drop(tx.rollback().await);
                Err(e)
            }
        }
    }
    async fn get_user_from_id(&self, user_id: i64) -> Result<User, Error> {
        let sql = "SELECT * FROM users WHERE user_id = ?";
        query_as::<_, User>(sql)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::UserNotFound)
    }

    async fn get_user_authentication_from_id(&self, user_id: UserId) -> Result<UserAuthentication, Error> {
        let sql = "SELECT * FROM user_authentication WHERE user_id = ?";
        query_as::<_, UserAuthentication>(sql)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::UserNotFound)
    }
    async fn get_user_profile_from_username(&self, username: &str) -> Result<UserProfile, Error> {
        let sql = "SELECT * FROM user_profiles WHERE username = ?";
        query_as::<_, UserProfile>(sql)
            .bind(username)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::UserNotFound)
    }
    async fn get_user_profile_from_id(&self, user_id: UserId) -> Result<UserProfile, Error> {
        let sql = "SELECT * FROM user_profiles WHERE user_id = ?";
        query_as::<_, UserProfile>(sql)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::UserNotFound)
    }
    async fn get_user_compact_from_id(&self, user_id: UserId) -> Result<UserCompact, Error> {
        let sql = "SELECT tu.user_id, tp.username, tu.administrator FROM users tu INNER JOIN user_profiles tp ON tu.user_id = tp.user_id WHERE tu.user_id = ?";
        query_as::<_, UserCompact>(sql)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::UserNotFound)
    }
    async fn count_users(&self) -> Result<i64, Error> {
        let sql = "SELECT COUNT(*) as count FROM users";
        query_as(sql)
            .fetch_one(&self.pool)
            .await
            .map(|(v,)| v)
            .map_err(|_| Error::Error)
    }
    async fn ban_user(&self, user_id: i64, reason: &str, date_expiry: NaiveDateTime) -> Result<(), Error> {
        // date needs to be in ISO 8601 format
        let date_expiry_string = date_expiry.format("%Y-%m-%d %H:%M:%S").to_string();
        let sql = "INSERT INTO user_bans (user_id, reason, date_expiry) VALUES ($1, $2, $3)";
        query(sql)
            .bind(user_id)
            .bind(reason)
            .bind(date_expiry_string)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|_| Error::Error)
    }
    async fn grant_admin_role(&self, user_id: UserId) -> Result<(), Error> {
        let sql = "UPDATE users SET administrator = TRUE WHERE user_id = ?";
        query(sql)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::UserNotFound)
                }
            })
    }
    async fn verify_email(&self, user_id: UserId) -> Result<(), Error> {
        let sql = "UPDATE user_profiles SET email_verified = TRUE WHERE user_id = ?";
        query(sql)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|_| Error::Error)
    }
    async fn delete_user(&self, user_id: UserId) -> Result<(), Error> {
        let sql = "DELETE FROM users WHERE user_id = ?";
        query(sql)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::UserNotFound)
                }
            })
    }
    async fn insert_room_and_get_id(&self, name: &str) -> Result<i64, Error> {
        let sql = "INSERT INTO rooms (name) VALUES (?)";
        query(sql)
            .bind(name)
            .execute(&self.pool)
            .await
            .map(|v| v.last_insert_rowid())
            .map_err(|_| Error::Error)
    }
    async fn insert_room_with_desc_and_get_id(&self, name: &str, desc: &str) -> Result<i64, Error> {
        let sql = "INSERT INTO rooms (name, description) VALUES (?, ?)";
        query(sql)
            .bind(name)
            .bind(desc)
            .execute(&self.pool)
            .await
            .map(|v| v.last_insert_rowid())
            .map_err(|_| Error::Error)
    }
    async fn delete_room(&self, room_id: RoomId) -> Result<(), Error> {
        let sql = "DELETE FROM rooms WHERE room_id = ?";
        query(sql)
            .bind(room_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::RoomNotFound)
                }
            })
    }
    async fn delete_rooms(&self, ids: &Vec<RoomId>) -> Result<BatchDelResult, Error> {
        todo!()
    }
    async fn update_room(&self, room_id: RoomId, name: &str, desc: &Option<String>) -> Result<(), Error> {
        let sql = "UPDATE rooms SET name = ?, description = ? WHERE room_id = ?";
        query(sql)
            .bind(name)
            .bind(desc)
            .bind(room_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::RoomNotFound)
                }
            })
    }
    async fn update_room_name(&self, room_id: RoomId, name: &str) -> Result<(), Error> {
        let sql = "UPDATE rooms SET name = ? WHERE room_id = ?";
        query(sql)
            .bind(name)
            .bind(room_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::RoomNotFound)
                }
            })
    }
    async fn update_room_desc(&self, room_id: RoomId, description: &str) -> Result<(), Error> {
        let sql = "UPDATE rooms SET description = ? WHERE room_id = ?";
        query(sql)
            .bind(description)
            .bind(room_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::RoomNotFound)
                }
            })
    }
    async fn get_room_from_id(&self, room_id: RoomId) -> Result<Room, Error> {
        let sql = "SELECT * FROM rooms WHERE room_id = ?";
        query_as::<_, Room>(sql)
            .bind(room_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::RoomNotFound)
    }
    async fn get_rooms(&self, offset: u64, limit: u8, sort: &Sorting) -> Result<Listing<Room>, Error> {
        let sql = "SELECT COUNT(*) as count FROM rooms";
        let count_result: Result<i64, Error> = query_as(sql)
            .fetch_one(&self.pool)
            .await
            .map(|(v,)| v)
            .map_err(|_| Error::Error);
        let count = count_result?;
        let sort_query: String = match sort {
            Sorting::NameAsc => "name ASC".to_string(),
            Sorting::NameDesc => "name DESC".to_string(),
            Sorting::IdAsc => "room_id ASC".to_string(),
            Sorting::IdDesc => "room_id DESC".to_string(),
        };
        let sql = format!("SELECT * FROM rooms ORDER BY {sort_query} LIMIT ?, ?");
        let rooms: Vec<Room> = query_as::<_, Room>(&sql)
            .bind(i64::saturating_add_unsigned(0, offset))
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| Error::Error)?;
        Ok(Listing {
            total: u64::try_from(count).expect("variable `count` is larger than u32"),
            data: rooms,
        })
    }
    async fn get_all_rooms(&self) -> Result<Vec<Room>, Error> {
        let sql = "SELECT * FROM rooms";
        let rooms: Vec<Room> = query_as::<_, Room>(&sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| Error::Error)?;
        Ok(rooms)
    }
    async fn insert_shelf_and_get_id(&self, name: &str, layer: i64, room_id: RoomId) -> Result<ShelfId, Error> {
        let sql = "INSERT INTO shelf (name, layer, room_id) VALUES (?, ?, ?)";
        query(sql)
            .bind(name)
            .bind(layer)
            .bind(room_id)
            .execute(&self.pool)
            .await
            .map(|v| v.last_insert_rowid())
            .map_err(|err| match err {
                sqlx::Error::PoolClosed => Error::Error,
                _ => Error::Error,
            })
    }
    async fn delete_shelf(&self, shelf_id: ShelfId) -> Result<(), Error> {
        let sql = "DELETE FROM shelf WHERE shelf_id = ?";
        query(sql)
            .bind(shelf_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::ShelfNotFound)
                }
            })
    }
    async fn delete_shelves(&self, ids: &Vec<ShelfId>) -> Result<BatchDelResult, Error> {
        todo!()
    }
    async fn update_shelf(&self, shelf_id: ShelfId, name: &str, layer: i64, room_id: RoomId) -> Result<(), Error> {
        let sql = "UPDATE shelf SET name = ?, layer = ?, room_id = ? WHERE shelf_id = ?";
        query(sql)
            .bind(name)
            .bind(layer)
            .bind(room_id)
            .bind(shelf_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::ShelfNotFound)
                }
            })
    }
    async fn update_shelf_name(&self, shelf_id: ShelfId, name: &str) -> Result<(), Error> {
        let sql = "UPDATE shelf SET name = ? WHERE shelf_id = ?";
        query(sql)
            .bind(name)
            .bind(shelf_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::ShelfNotFound)
                }
            })
    }
    async fn update_shelf_layer(&self, shelf_id: ShelfId, layer: i64) -> Result<(), Error> {
        let sql = "UPDATE shelf SET layer = ? WHERE shelf_id = ?";
        query(sql)
            .bind(layer)
            .bind(shelf_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::ShelfNotFound)
                }
            })
    }
    async fn update_shelf_room(&self, shelf_id: ShelfId, room_id: RoomId) -> Result<(), Error> {
        let sql = "UPDATE shelf SET room_id = ? WHERE shelf_id = ?";
        query(sql)
            .bind(room_id)
            .bind(shelf_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::ShelfNotFound)
                }
            })
    }
    async fn get_shelf_from_id(&self, shelf_id: ShelfId) -> Result<Shelf, Error> {
        let sql = "SELECT * FROM shelf WHERE shelf_id = ?";
        query_as::<_, Shelf>(sql)
            .bind(shelf_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::ShelfNotFound)
    }
    async fn get_shelves(&self, offset: u64, limit: u8, sort: &Sorting) -> Result<Listing<Shelf>, Error> {
        let sql = "SELECT COUNT(*) as count FROM shelf";
        let count_result: Result<i64, Error> = query_as(sql)
            .fetch_one(&self.pool)
            .await
            .map(|(v,)| v)
            .map_err(|_| Error::Error);
        let count = count_result?;
        let sort_query: String = match sort {
            Sorting::NameAsc => "name ASC".to_string(),
            Sorting::NameDesc => "name DESC".to_string(),
            Sorting::IdAsc => "shelf_id ASC".to_string(),
            Sorting::IdDesc => "shelf_id DESC".to_string(),
        };
        let sql = format!("SELECT * FROM shelf ORDER BY {sort_query} LIMIT ?, ?");
        let shelves: Vec<Shelf> = query_as::<_, Shelf>(&sql)
            .bind(i64::saturating_add_unsigned(0, offset))
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| database::Error::Error)?;
        Ok(Listing {
            total: u64::try_from(count).expect("variable `count` is larger than u32"),
            data: shelves,
        })
    }
    async fn get_shelves_in_room(
        &self,
        offset: u64,
        limit: u8,
        sort: &Sorting,
        room_id: RoomId,
    ) -> Result<Listing<Shelf>, Error> {
        let sql = "SELECT COUNT(*) as count FROM shelf WHERE room_id = ?";
        let count_result: Result<i64, Error> = query_as(sql)
            .bind(room_id)
            .fetch_one(&self.pool)
            .await
            .map(|(v,)| v)
            .map_err(|_| Error::Error);
        let count = count_result?;
        let sort_query: String = match sort {
            Sorting::NameAsc => "name ASC".to_string(),
            Sorting::NameDesc => "name DESC".to_string(),
            Sorting::IdAsc => "shelf_id ASC".to_string(),
            Sorting::IdDesc => "shelf_id DESC".to_string(),
        };
        let sql = format!("SELECT * FROM shelf WHERE room_id = ? ORDER BY {sort_query} LIMIT ?, ?");
        let shelves: Vec<Shelf> = query_as::<_, Shelf>(&sql)
            .bind(room_id)
            .bind(i64::saturating_add_unsigned(0, offset))
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| database::Error::Error)?;
        Ok(Listing {
            total: u64::try_from(count).expect("variable `count` is larger than u32"),
            data: shelves,
        })
    }
    async fn get_all_shelves(&self) -> Result<Vec<Shelf>, Error> {
        let sql = "SELECT * FROM shelf";
        let shelves: Vec<Shelf> = query_as::<_, Shelf>(&sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| Error::Error)?;
        Ok(shelves)
    }
    async fn get_all_shelves_in_room(&self, room_id: RoomId) -> Result<Vec<Shelf>, Error> {
        let sql = "SELECT * FROM shelf WHERE room_id = ?";
        let shelves: Vec<Shelf> = query_as::<_, Shelf>(&sql)
            .bind(room_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| Error::Error)?;
        Ok(shelves)
    }
    async fn insert_item_and_get_id(&self, name: &str, sn: &str) -> Result<ItemId, Error> {
        let sql = "INSERT INTO items (name, sn) VALUES (?, ?)";
        query(sql)
            .bind(name)
            .bind(sn)
            .execute(&self.pool)
            .await
            .map(|v| v.last_insert_rowid())
            .map_err(|_| Error::Error)
    }
    async fn insert_item_with_desc_and_get_id(&self, name: &str, desc: &str, sn: &str) -> Result<ItemId, Error> {
        let sql = "INSERT INTO items (name, sn, description) VALUES (?, ?, ?)";
        query(sql)
            .bind(name)
            .bind(sn)
            .bind(desc)
            .execute(&self.pool)
            .await
            .map(|v| v.last_insert_rowid())
            .map_err(|_| Error::Error)
    }
    async fn delete_item(&self, item_id: ItemId) -> Result<(), Error> {
        let sql = "DELETE FROM items WHERE item_id = ?";
        query(sql)
            .bind(item_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::ItemNotFound)
                }
            })
    }
    async fn delete_items(&self, ids: &Vec<ItemId>) -> Result<BatchDelResult, Error> {
        todo!()
    }
    async fn update_item(&self, item_id: ItemId, name: &str, desc: &Option<String>, sn: &str) -> Result<(), Error> {
        let sql = "UPDATE items SET name = ?, description = ?, sn = ? WHERE item_id = ?";
        query(sql)
            .bind(name)
            .bind(desc)
            .bind(sn)
            .bind(item_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::ItemNotFound)
                }
            })
    }
    async fn update_item_name(&self, item_id: ItemId, name: &str) -> Result<(), Error> {
        let sql = "UPDATE items SET name = ? WHERE item_id = ?";
        query(sql)
            .bind(name)
            .bind(item_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::ItemNotFound)
                }
            })
    }
    async fn update_item_desc(&self, item_id: ItemId, desc: &str) -> Result<(), Error> {
        let sql = "UPDATE items SET description = ? WHERE item_id = ?";
        query(sql)
            .bind(desc)
            .bind(item_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::ItemNotFound)
                }
            })
    }
    async fn update_item_sn(&self, item_id: ItemId, sn: &str) -> Result<(), Error> {
        let sql = "UPDATE items SET sn = ? WHERE item_id = ?";
        query(sql)
            .bind(sn)
            .bind(item_id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| {
                if v.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(Error::ItemNotFound)
                }
            })
    }
    async fn get_item_from_id(&self, item_id: ItemId) -> Result<Item, Error> {
        let sql = "SELECT * FROM items WHERE item_id = ?";
        query_as::<_, Item>(sql)
            .bind(item_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::ItemNotFound)
    }
    async fn get_items(&self, offset: u64, limit: u8, sort: &Sorting) -> Result<Listing<Item>, Error> {
        let sql = "SELECT COUNT(*) as count FROM items";
        let count_result: Result<i64, Error> = query_as(sql)
            .fetch_one(&self.pool)
            .await
            .map(|(v,)| v)
            .map_err(|_| Error::Error);
        let count = count_result?;
        let sort_query: String = match sort {
            Sorting::NameAsc => "name ASC".to_string(),
            Sorting::NameDesc => "name DESC".to_string(),
            Sorting::IdAsc => "item_id ASC".to_string(),
            Sorting::IdDesc => "item_id DESC".to_string(),
        };
        let sql = format!("SELECT * FROM items ORDER BY {sort_query} LIMIT ?, ?");
        let items: Vec<Item> = query_as::<_, Item>(&sql)
            .bind(i64::saturating_add_unsigned(0, offset))
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| database::Error::Error)?;
        Ok(Listing {
            total: u64::try_from(count).expect("variable `count` is larger than u32"),
            data: items,
        })
    }
    async fn get_all_items(&self) -> Result<Vec<Item>, Error> {
        let sql = "SELECT * FROM items";
        let items: Vec<Item> = query_as::<_, Item>(&sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| Error::Error)?;
        Ok(items)
    }
    async fn get_stocks_on_shelves(&self, offset: u64, limit: u8, sort: &Sorting) -> Result<Listing<ItemOnShelf>, Error> {
        todo!()
    }
    async fn get_stocks_on_shelf(
        &self,
        offset: u64,
        limit: u8,
        sort: &Sorting,
        shelf_id: ShelfId,
    ) -> Result<Listing<ItemOnShelf>, Error> {
        let sql = "SELECT COUNT(*) as count
FROM stock si
         JOIN items it ON it.item_id = si.item_id
         JOIN shelf sf ON si.shelf_id = sf.shelf_id
WHERE si.shelf_id = ?";
        let count_result: Result<i64, Error> = query_as(sql)
            .bind(shelf_id)
            .fetch_one(&self.pool)
            .await
            .map(|(v,)| v)
            .map_err(|_| Error::Error);
        let count = count_result?;
        let sql = "SELECT si.item_id  item_id,
       it.name     item_name,
       si.shelf_id shelf_id,
       sf.name     shelf_name,
       si.count    count,
       it.sn       sn
FROM stock si
         JOIN items it ON it.item_id = si.item_id
         JOIN shelf sf ON si.shelf_id = sf.shelf_id
WHERE si.shelf_id = ? LIMIT ?, ?";
        let items: Vec<ItemOnShelf> = query_as::<_, ItemOnShelf>(&sql)
            .bind(shelf_id)
            .bind(i64::saturating_add_unsigned(0, offset))
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| database::Error::Error)?;
        Ok(Listing {
            total: u64::try_from(count).expect("variable `count` is larger than u32"),
            data: items,
        })
    }
    async fn get_stocks_in_rooms(&self, offset: u64, limit: u8, sort: &Sorting) -> Result<Listing<ItemInRoom>, Error> {
        todo!()
    }
    async fn get_stocks_in_room(
        &self,
        offset: u64,
        limit: u8,
        sort: &Sorting,
        room_id: RoomId,
    ) -> Result<Listing<ItemInRoom>, Error> {
        let sql = "SELECT COUNT(*) count
FROM (SELECT SUM(si.count) inner_count
      FROM stock si
               JOIN items it ON it.item_id = si.item_id
               JOIN shelf sf ON si.shelf_id = sf.shelf_id
               JOIN rooms r ON sf.room_id = r.room_id
      WHERE r.room_id = ?
      GROUP BY it.item_id)";
        let count_result: Result<i64, Error> = query_as(sql)
            .fetch_one(&self.pool)
            .await
            .map(|(v,)| v)
            .map_err(|_| Error::Error);
        let count = count_result?;
        let sql = "SELECT it.item_id, it.name item_name,SUM(si.count) count, r.room_id, r.name room_name
FROM stock si
         JOIN items it ON it.item_id = si.item_id
         JOIN shelf sf ON si.shelf_id = sf.shelf_id
         JOIN rooms r ON sf.room_id = r.room_id
WHERE r.room_id = ? GROUP BY it.item_id LIMIT ?, ?";
        let items: Vec<ItemInRoom> = query_as::<_, ItemInRoom>(&sql)
            .bind(room_id)
            .bind(i64::saturating_add_unsigned(0, offset))
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| database::Error::Error)?;
        Ok(Listing {
            total: u64::try_from(count).expect("variable `count` is larger than u32"),
            data: items,
        })
    }
    async fn transfer_items(&self, item_id: ItemId, count: i64, shelf_from: ShelfId, shelf_to: ShelfId) -> Result<(), Error> {
        if count <= 0 {
            return Err(Error::CountMustBePositive);
        }
        let mut conn = self.pool.acquire().await.map_err(|_| Error::ConnectionPoolFailed)?;
        let mut tx = conn.begin().await.map_err(|_| Error::TransactionError)?;
        let sql = "SELECT * FROM stock WHERE item_id = ? and shelf_id = ?";
        let x_from = query_as::<_, ItemXShelf>(sql)
            .bind(item_id)
            .bind(shelf_from)
            .fetch_one(&mut *tx)
            .await
            .map_err(|_| Error::InsufficientItem)?;
        let new_from_count = x_from.count - count;
        if new_from_count <= 0 {
            drop(tx.rollback().await);
            return Err(Error::InsufficientItem);
        }
        let update_sql = "UPDATE stock SET count = ? WHERE item_id = ? and shelf_id = ?";
        let update_res = query(update_sql)
            .bind(new_from_count)
            .bind(item_id)
            .bind(shelf_from)
            .execute(&mut *tx)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| if v.rows_affected() > 0 { Ok(()) } else { Err(Error::Error) });
        if update_res.is_err() {
            drop(tx.rollback().await);
            return update_res;
        }
        let x_to_res = query_as::<_, ItemXShelf>(update_sql)
            .bind(item_id)
            .bind(shelf_to)
            .fetch_one(&mut *tx)
            .await;
        if let Ok(x_to) = x_to_res {
            let update_res = query(update_sql)
                .bind(x_to.count + count)
                .bind(item_id)
                .bind(shelf_to)
                .execute(&mut *tx)
                .await
                .map_err(|_| Error::Error)
                .and_then(|v| if v.rows_affected() > 0 { Ok(()) } else { Err(Error::Error) });
            match update_res {
                Ok(_) => {
                    drop(tx.commit().await);
                    Ok(())
                }
                Err(e) => {
                    drop(tx.rollback().await);
                    Err(e)
                }
            }
        } else {
            let sql = "INSERT INTO stock (count, item_id, shelf_id) VALUES (?, ?, ?)";
            let insert_result = query(sql)
                .bind(count)
                .bind(item_id)
                .bind(shelf_to)
                .execute(&mut *tx)
                .await
                .map_err(|_| Error::Error)
                .and_then(|v| if v.rows_affected() > 0 { Ok(()) } else { Err(Error::Error) });
            match insert_result {
                Ok(_) => {
                    drop(tx.commit().await);
                    Ok(())
                }
                Err(e) => {
                    drop(tx.rollback().await);
                    Err(e)
                }
            }
        }
    }
    async fn deposit_items(&self, item_id: ItemId, count: i64, shelf_id: ShelfId) -> Result<(), Error> {
        if count <= 0 {
            return Err(Error::CountMustBePositive);
        }
        let mut conn = self.pool.acquire().await.map_err(|_| Error::ConnectionPoolFailed)?;
        let mut tx = conn.begin().await.map_err(|_| Error::TransactionError)?;
        let sql = "SELECT * FROM stock WHERE item_id = ? and shelf_id = ?";
        let x_res = query_as::<_, ItemXShelf>(sql)
            .bind(item_id)
            .bind(shelf_id)
            .fetch_one(&mut *tx)
            .await;
        let new_count;
        let sql = if let Ok(x) = x_res {
            new_count = x.count + count;
            "UPDATE stock SET count = ? WHERE item_id = ? and shelf_id = ?"
        } else {
            new_count = count;
            "INSERT INTO stock (count, item_id, shelf_id) VALUES (?, ?, ?)"
        };
        let upsert_result = query(sql)
            .bind(new_count)
            .bind(item_id)
            .bind(shelf_id)
            .execute(&mut *tx)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| if v.rows_affected() > 0 { Ok(()) } else { Err(Error::Error) });
        match upsert_result {
            Ok(_) => {
                drop(tx.commit().await);
                Ok(())
            }
            Err(e) => {
                drop(tx.rollback().await);
                Err(e)
            }
        }
    }
    async fn withdraw_items(&self, item_id: ItemId, count: i64, shelf_id: ShelfId) -> Result<(), Error> {
        if count <= 0 {
            return Err(Error::CountMustBePositive);
        }
        let mut conn = self.pool.acquire().await.map_err(|_| Error::ConnectionPoolFailed)?;
        let mut tx = conn.begin().await.map_err(|_| Error::TransactionError)?;
        let sql = "SELECT * FROM stock WHERE item_id = ? and shelf_id = ?";
        let x = query_as::<_, ItemXShelf>(sql)
            .bind(item_id)
            .bind(shelf_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|_| Error::InsufficientItem)?;
        let new_count = x.count - count;
        if new_count <= 0 {
            drop(tx.rollback().await);
            return Err(Error::InsufficientItem);
        }
        let sql = "UPDATE stock SET count = ? WHERE item_id = ? and shelf_id = ?";
        let update_result = query(sql)
            .bind(new_count)
            .bind(item_id)
            .bind(shelf_id)
            .execute(&mut *tx)
            .await
            .map_err(|_| Error::Error)
            .and_then(|v| if v.rows_affected() > 0 { Ok(()) } else { Err(Error::Error) });
        match update_result {
            Ok(_) => {
                drop(tx.commit().await);
                Ok(())
            }
            Err(e) => {
                drop(tx.rollback().await);
                Err(e)
            }
        }
    }

    async fn convert_items(&self, from: Vec<ItemXShelf>, into: Vec<ItemXShelf>) -> Result<(), Error> {
        // todo, insufficient item must be more clear
        let mut conn = self.pool.acquire().await.map_err(|_| Error::ConnectionPoolFailed)?;
        let mut tx = conn.begin().await.map_err(|_| Error::TransactionError)?;
        let select_sql = "SELECT * FROM stock WHERE item_id = ? and shelf_id = ?";
        let update_sql = "UPDATE stock SET count = ? WHERE item_id = ? and shelf_id = ?";
        let insert_sql = "INSERT INTO stock (count, item_id, shelf_id) VALUES (?, ?, ?)";
        for x_from in from {
            if x_from.count <= 0 {
                drop(tx.rollback().await);
                return Err(Error::CountMustBePositive);
            }
            let x_res = query_as::<_, ItemXShelf>(select_sql)
                .bind(x_from.item_id)
                .bind(x_from.shelf_id)
                .fetch_one(&mut *tx)
                .await
                .map_err(|_| Error::InsufficientItem);
            if let Err(err) = x_res {
                drop(tx.rollback().await);
                return Err(err);
            }
            let x = x_res.unwrap();
            let new_count = x.count - x_from.count;
            if new_count <= 0 {
                drop(tx.rollback().await);
                return Err(Error::InsufficientItem);
            }
            let update_result = query(update_sql)
                .bind(new_count)
                .bind(x_from.item_id)
                .bind(x_from.shelf_id)
                .execute(&mut *tx)
                .await
                .map_err(|_| Error::Error)
                .and_then(|v| if v.rows_affected() > 0 { Ok(()) } else { Err(Error::Error) });
            if let Err(err) = update_result {
                drop(tx.rollback().await);
                return Err(err);
            }
        }
        for x_into in into {
            if x_into.count <= 0 {
                drop(tx.rollback().await);
                return Err(Error::CountMustBePositive);
            }
            let x_res = query_as::<_, ItemXShelf>(select_sql)
                .bind(x_into.item_id)
                .bind(x_into.shelf_id)
                .fetch_one(&mut *tx)
                .await;
            let new_count;
            let sql;
            if let Ok(old_x) = x_res {
                new_count = old_x.count + x_into.count;
                sql = update_sql;
            } else {
                new_count = x_into.count;
                sql = insert_sql;
            }
            let upsert_result = query(sql)
                .bind(new_count)
                .bind(x_into.item_id)
                .bind(x_into.shelf_id)
                .execute(&mut *tx)
                .await
                .map_err(|_| Error::Error)
                .and_then(|v| if v.rows_affected() > 0 { Ok(()) } else { Err(Error::Error) });
            if let Err(err) = upsert_result {
                drop(tx.rollback().await);
                return Err(err);
            }
        }
        drop(tx.commit().await);
        Ok(())
    }
}
