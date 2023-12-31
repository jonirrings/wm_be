use std::str::FromStr;
use std::time::Duration;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{query, query_as, Acquire, ConnectOptions, SqlitePool};

use crate::databases::database::{ Database, Driver, Sorting};

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
}