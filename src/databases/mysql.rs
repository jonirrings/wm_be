use std::str::FromStr;
use std::time::Duration;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{query, query_as, Acquire, ConnectOptions, MySqlPool};

use crate::databases::database::{ Database, Driver, Sorting};

pub struct Mysql {
    pub pool: MySqlPool,
}

#[async_trait]
impl Database for Mysql {
    fn get_database_driver(&self) -> Driver {
        Driver::Mysql
    }
    async fn new(database_url: &str) -> Self {
        let connection_options = MySqlConnectOptions::from_str(database_url)
            .expect("Unable to create connection options.")
            .log_statements(log::LevelFilter::Debug)
            .log_slow_statements(log::LevelFilter::Info, Duration::from_secs(1));

        let db = MySqlPoolOptions::new()
            .connect_with(connection_options)
            .await
            .expect("Unable to create database pool.");

        sqlx::migrate!("migrations/mysql")
            .run(&db)
            .await
            .expect("Could not run database migrations.");

        Self { pool: db }
    }
}