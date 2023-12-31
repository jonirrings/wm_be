use std::sync::Arc;
use crate::databases::database::Database;
use crate::config::Configuration;

pub struct AppData {
    pub cfg: Arc<Configuration>,
    pub database: Arc<Box<dyn Database>>,
}

impl AppData {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cfg: Arc<Configuration>,
        database: Arc<Box<dyn Database>>) -> Self {
        AppData {
            cfg,
            database,
        }
    }
}