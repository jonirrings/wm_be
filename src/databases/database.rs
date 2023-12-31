use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::databases::mysql::Mysql;
use crate::databases::sqlite::Sqlite;
use crate::databases::postgres::Postgres;

/// Database drivers.
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum Driver {
    Sqlite3,
    Mysql,
    Postgres,
}

/// Sorting options for Item.
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Sorting {
    NameAsc,
    NameDesc,
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
}

/// Get the Driver of the Database from the Connection String
///
/// # Errors
///
/// This function will return an `Error::UnrecognizedDatabaseDriver` if unable to match database type.
pub fn get_driver(db_path: &str) -> Result<Driver, Error> {
    match &db_path.chars().collect::<Vec<char>>() as &[char] {
        ['s', 'q', 'l', 'i', 't', 'e', ..] => Ok(Driver::Sqlite3),
        ['m', 'y', 's', 'q', 'l', ..] => Ok(Driver::Mysql),
        ['p', 'o', 's', 't', 'g', 'r', 'e', 's', 'q', 'l', ..] => Ok(Driver::Postgres),
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
        Driver::Mysql => Box::new(Mysql::new(db_path).await),
        Driver::Postgres => Box::new(Postgres::new(db_path).await),
    })
}

/// Trait for database implementations.
#[async_trait]
pub trait Database: Sync + Send {
    /// Return current database driver.
    fn get_database_driver(&self) -> Driver;
    async fn new(db_path: &str) -> Self where Self: Sized;
}