use std::path::Path;
use std::sync::Arc;
use std::{env, fs};

use crate::common::{ListingCriteria, ListingSpec};
use crate::databases::database::Sorting;
use config::{Config, ConfigError, File, FileFormat};
use located_error::{Located, LocatedError};
use log::warn;
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Debug, Default, Clone)]
pub struct Info {
    index: String,
}

impl Info {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(env_var_config: String, env_var_path_config: String, default_path_config: String) -> Result<Self, Error> {
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

        Ok(Self { index })
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

/// The base URL for the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub ip: Option<String>,
    /// The port to listen on. Default to `3001`.
    pub port: u16,
    /// The base URL for the API. For example: `http://localhost`.
    /// If not set, the base URL will be inferred from the request.
    pub base_url: Option<String>,
}

impl Default for Network {
    fn default() -> Self {
        Self {
            ip: None,
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
            connect_url: "sqlite://wm.sqlite?mode=rwc".to_string(),
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
    pub async fn load_from_file(config_path: &str) -> Result<Configuration, ConfigError> {
        let config_builder = Config::builder();

        #[allow(unused_assignments)]
        let mut config = Config::default();

        if Path::new(config_path).exists() {
            config = config_builder.add_source(File::with_name(config_path)).build()?;
        } else {
            warn!("No config file found. Creating default config file ...");

            let config = Configuration::default();
            let () = config.save_to_file(config_path).await;

            return Err(ConfigError::Message(format!(
                "No config file found. Created default config file in {config_path}. Edit the file and start the application."
            )));
        }

        let wm_config: WarehouseIndex = match config.try_deserialize() {
            Ok(data) => Ok(data),
            Err(e) => Err(ConfigError::Message(format!("Errors while processing config: {e}."))),
        }?;

        Ok(Configuration {
            settings: RwLock::new(wm_config),
            config_path: Some(config_path.to_string()),
        })
    }
    pub fn load(info: &Info) -> Result<Configuration, Error> {
        let config_builder = Config::builder()
            .add_source(File::from_str(&info.index, FileFormat::Toml))
            .build()
            .unwrap(); //fixme
        let index_config: WarehouseIndex = config_builder.try_deserialize().unwrap(); //fixme

        Ok(Configuration {
            settings: RwLock::new(index_config),
            config_path: None,
        })
    }

    /// Returns the save to file of this [`Configuration`].
    ///
    /// # Panics
    ///
    /// This function will panic if it can't write to the file.
    pub async fn save_to_file(&self, config_path: &str) {
        let settings = self.settings.read().await;

        let toml_string = toml::to_string(&*settings).expect("Could not encode TOML value");

        drop(settings);

        fs::write(config_path, toml_string).expect("Could not write to file!");
    }

    pub async fn get_all(&self) -> WarehouseIndex {
        let settings_lock = self.settings.read().await;

        settings_lock.clone()
    }

    pub async fn get_public(&self) -> ConfigurationPublic {
        let settings_lock = self.settings.read().await;

        ConfigurationPublic {
            website_name: settings_lock.website.name.clone(),
            email_on_signup: settings_lock.auth.email_on_signup.clone(),
        }
    }

    pub async fn get_site_name(&self) -> String {
        let settings_lock = self.settings.read().await;

        settings_lock.website.name.clone()
    }

    pub async fn get_api_base_url(&self) -> Option<String> {
        let settings_lock = self.settings.read().await;

        settings_lock.net.base_url.clone()
    }
    pub async fn spec_from_criteria(&self, request: &ListingCriteria) -> ListingSpec {
        let settings = self.settings.read().await;
        let default_page_size = settings.api.default_page_size;
        let max_page_size = settings.api.max_page_size;
        drop(settings);
        let sort = request.sort.unwrap_or(Sorting::IdDesc);
        let offset = request.offset.unwrap_or(0);
        let limit = request.limit.unwrap_or(default_page_size);
        let limit = if limit > max_page_size { max_page_size } else { limit };
        ListingSpec { offset, limit, sort }
    }
}

/// The public index configuration.
/// There is an endpoint to get this configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationPublic {
    website_name: String,
    email_on_signup: EmailOnSignup,
}
