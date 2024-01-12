use std::str::FromStr;
use std::time::Duration;
use std::u64;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{query, query_as, Acquire, ConnectOptions, SqlitePool};

use crate::databases::database::{self, Database, Driver, Error, Rooms, Sorting};
use crate::models::room::{Room, RoomId};
use crate::models::user::{User, UserAuthentication, UserCompact, UserId, UserProfile};

pub struct Sqlite {
    pub pool: SqlitePool,
}

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
        let mut conn = self.pool.acquire().await.map_err(|_| Error::Error)?;
        // start db transaction
        let mut tx = conn.begin().await.map_err(|_| Error::Error)?;
        // create the user account and get the user id
        let sql = "INSERT INTO users (created_at) VALUES (strftime('%Y-%m-%d %H:%M:%S',DATETIME('now', 'utc')))";
        let user_id = query(sql).execute(&mut *tx).await.map(|v| v.last_insert_rowid()).map_err(|_| Error::Error)?;
        // add password hash for account
        let sql = "INSERT INTO user_authentication (user_id, password_hash) VALUES (?, ?)";
        let insert_user_auth_result = query(sql).bind(user_id).bind(password_hash).execute(&mut *tx).await.map_err(|_| Error::Error);
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
                _ => Error::Error
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
            .map(|(v, )| v)
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
    async fn insert_room_with_desc_and_get_id(&self, name: &str, description: &str) -> Result<i64, Error> {
        let sql = "INSERT INTO rooms (name, description) VALUES (?, ?)";
        query(sql)
            .bind(name)
            .bind(description)
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
    async fn get_rooms(&self, offset: u64, limit: u8, sort: &Sorting) -> Result<Rooms, Error> {
        let sql = "SELECT COUNT(*) as count FROM rooms";
        let count_result:Result<i64,Error> = query_as(sql)
            .fetch_one(&self.pool)
            .await
            .map(|(v, )| v)
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
            .map_err(|_| database::Error::Error)?;
        Ok(Rooms {
            total: u64::try_from(count).expect("variable `count` is larger than u32"),
            data: rooms,
        })
    }
}