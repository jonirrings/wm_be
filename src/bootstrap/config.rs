use crate::config::{Configuration, Info};

/// The whole `index.toml` file content. It has priority over the config file.
/// Even if the file is not on the default path.
const ENV_VAR_CONFIG: &str = "INDEX_CONFIG";

/// The `index.toml` file location.
pub const ENV_VAR_PATH_CONFIG: &str = "INDEX_PATH_CONFIG";

// Default values
pub const DEFAULT_PATH_CONFIG: &str = "./share/default/config/index.development.sqlite3.toml";
/// If present, CORS will be permissive.
pub const ENV_VAR_CORS_PERMISSIVE: &str = "WM_API_CORS_PERMISSIVE";

#[must_use]
pub fn initialize_configuration() -> Configuration {
    let info = Info::new(
        ENV_VAR_CONFIG.to_string(),
        ENV_VAR_PATH_CONFIG.to_string(),
        DEFAULT_PATH_CONFIG.to_string(),
    )
        .unwrap();

    Configuration::load(&info).unwrap()
}