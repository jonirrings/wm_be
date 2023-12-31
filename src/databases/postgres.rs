use std::str::FromStr;
use std::time::Duration;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::postgres::{PgConnectOptions,PgPoolOptions};
use sqlx::{query, query_as, Acquire, ConnectOptions, PgPool};

use crate::databases::database::{ Database, Driver, Sorting};

pub struct Postgres {
    pub pool: PgPool,
}
#[async_trait]
impl Database for Postgres {
    fn get_database_driver(&self) -> Driver {
        Driver::Postgres
    }
    async fn new(database_url: &str) -> Self {
        let connection_options = PgConnectOptions::from_str(database_url)
            .expect("Unable to create connection options.")
            .log_statements(log::LevelFilter::Debug)
            .log_slow_statements(log::LevelFilter::Info, Duration::from_secs(1));

        let db = PgPoolOptions::new()
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