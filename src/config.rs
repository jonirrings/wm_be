use std::path::Path;
use std::sync::Arc;
use std::{env, fs};

use config::{Config, ConfigError, File, FileFormat};
use log::warn;
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::RwLock;
use located_error::{Located, LocatedError};

#[derive(Debug, Default, Clone)]
pub struct Info {
    index: String,
}

impl Info {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(env_var_config: String,
               env_var_path_config: String,
               default_path_config: String) -> Result<Self, Error> {
        let index = if let Ok(index) = env::var(&env_var_config) {
            println!("Loading configuration from env var {env_var_config} ...");

            index
        } else {
            let config_path = if let Ok(config_path) = env::var(env_var_path_config) {
                println!("Loading configuration file: `{config_path}` ...");

                config_path
            } else {
                println!("Loading default configuration file: `{default_path_config}` ...");

                default_path_config
            };

            fs::read_to_string(config_path)
                .map_err(|e| Error::UnableToLoadFromConfigFile {
                    source: (Arc::new(e) as Arc<dyn std::error::Error + Send + Sync>).into(),
                })?
                .parse()
                .map_err(|_e: std::convert::Infallible| Error::Infallible)?
        };

        Ok(Self {
            index,
        })
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to load from Environmental Variable: {source}")]
    UnableToLoadFromEnvironmentVariable {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Unable to load from Config File: {source}")]
    UnableToLoadFromConfigFile {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    /// Unable to load the configuration from the configuration file.
    #[error("Failed processing the configuration: {source}")]
    ConfigError { source: LocatedError<'static, ConfigError> },

    #[error("The error for errors that can never happen.")]
    Infallible,
}

/// Information displayed to the user in the website.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Website {
    /// The name of the website.
    pub name: String,
}

impl Default for Website {
    fn default() -> Self {
        Self {
            name: "Warehouse Management".to_string(),
        }
    }
}

/// Port number representing that the OS will choose one randomly from the available ports.
///
/// It's the port number `0`
pub const FREE_PORT: u16 = 0;

/// The the base URL for the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    /// The port to listen on. Default to `3001`.
    pub port: u16,
    /// The base URL for the API. For example: `http://localhost`.
    /// If not set, the base URL will be inferred from the request.
    pub base_url: Option<String>,
}

impl Default for Network {
    fn default() -> Self {
        Self {
            port: 6001,
            base_url: None,
        }
    }
}

/// Whether the email is required on signup or not.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmailOnSignup {
    /// The email is required on signup.
    Required,
    /// The email is optional on signup.
    Optional,
    /// The email is not allowed on signup. It will only be ignored if provided.
    None, // code-review: rename to `Ignored`?
}

impl Default for EmailOnSignup {
    fn default() -> Self {
        Self::Optional
    }
}

/// Authentication options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth {
    /// Whether or not to require an email on signup.
    pub email_on_signup: EmailOnSignup,
    /// The minimum password length.
    pub min_password_length: usize,
    /// The maximum password length.
    pub max_password_length: usize,
    /// The secret key used to sign JWT tokens.
    pub secret_key: String,
}

impl Default for Auth {
    fn default() -> Self {
        Self {
            email_on_signup: EmailOnSignup::default(),
            min_password_length: 6,
            max_password_length: 64,
            secret_key: "MaxVerstappenWC2021".to_string(),
        }
    }
}

/// Database configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    /// The connection string for the database. For example: `sqlite://data.db?mode=rwc`.
    pub connect_url: String,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            connect_url: "sqlite://data.db?mode=rwc".to_string(),
        }
    }
}

/// SMTP configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mail {
    /// Whether or not to enable email verification on signup.
    pub email_verification_enabled: bool,
    /// The email address to send emails from.
    pub from: String,
    /// The email address to reply to.
    pub reply_to: String,
    /// The username to use for SMTP authentication.
    pub username: String,
    /// The password to use for SMTP authentication.
    pub password: String,
    /// The SMTP server to use.
    pub server: String,
    /// The SMTP port to use.
    pub port: u16,
}

impl Default for Mail {
    fn default() -> Self {
        Self {
            email_verification_enabled: false,
            from: "example@email.com".to_string(),
            reply_to: "noreply@email.com".to_string(),
            username: String::default(),
            password: String::default(),
            server: String::default(),
            port: 25,
        }
    }
}

/// Configuration for the image proxy cache.
///
/// Users have a cache quota per period. For example: 100MB per day.
/// When users are navigating the site, they will be downloading images that are
/// embedded in the item description. These images will be cached in the
/// proxy. The proxy will not download new images if the user has reached the
/// quota.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageCache {
    /// Maximum time in seconds to wait for downloading the image form the original source.
    pub max_request_timeout_ms: u64,
    /// Cache size in bytes.
    pub capacity: usize,
    /// Maximum size in bytes for a single image.
    pub entry_size_limit: usize,
    /// Users have a cache quota per period. For example: 100MB per day.
    /// This is the period in seconds (1 day in seconds).
    pub user_quota_period_seconds: u64,
    /// Users have a cache quota per period. For example: 100MB per day.
    /// This is the maximum size in bytes (100MB in bytes).
    pub user_quota_bytes: usize,
}

impl Default for ImageCache {
    fn default() -> Self {
        Self {
            max_request_timeout_ms: 1000,
            capacity: 128_000_000,
            entry_size_limit: 4_000_000,
            user_quota_period_seconds: 3600,
            user_quota_bytes: 64_000_000,
        }
    }
}

/// Core configuration for the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Api {
    /// The default page size for lists.
    pub default_page_size: u8,
    /// The maximum page size for lists.
    pub max_page_size: u8,
}

impl Default for Api {
    fn default() -> Self {
        Self {
            default_page_size: 10,
            max_page_size: 30,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WarehouseIndex {
    /// Logging level. Possible values are: `Off`, `Error`, `Warn`, `Info`,
    /// `Debug` and `Trace`. Default is `Info`.
    pub log_level: Option<String>,
    /// The website customizable values.
    pub website: Website,
    /// The network configuration.
    pub net: Network,
    /// The authentication configuration.
    pub auth: Auth,
    /// The database configuration.
    pub database: Database,
    /// The SMTP configuration.
    pub mail: Mail,
    /// The image proxy cache configuration.
    pub image_cache: ImageCache,
    /// The API configuration.
    pub api: Api,
}

/// The configuration service.
#[derive(Debug)]
pub struct Configuration {
    /// The state of the configuration.
    pub settings: RwLock<WarehouseIndex>,
    /// The path to the configuration file. This is `None` if the configuration
    /// was loaded from the environment.
    pub config_path: Option<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            settings: RwLock::new(WarehouseIndex::default()),
            config_path: None,
        }
    }
}

impl Configuration {
    pub async fn load_from_file(config_path: &str) { unimplemented!("load_from_file") }
    pub fn load(info: &Info) -> Result<Configuration, Error> {
        let config_builder = Config::builder()
            .add_source(File::from_str(&info.index, FileFormat::Toml))
            .build().unwrap();//fixme
        let index_config: WarehouseIndex = config_builder.try_deserialize().unwrap();//fixme

        Ok(Configuration {
            settings: RwLock::new(index_config),
            config_path: None,
        })
    }
    pub async fn save_to_file(&self, config_path: &str) { unimplemented!("save_to_file") }
    pub async fn get_all(&self) { unimplemented!("get_all") }
    pub async fn get_public(&self) { unimplemented!("get_public") }
    pub async fn get_site_name(&self) { unimplemented!("get_site_name") }
    pub async fn get_api_base_url(&self) { unimplemented!("get_api_base_url") }
}