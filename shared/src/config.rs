//! Implement the `Config` struct and related functions. This is used in the `ambient_download` and `tz_detect` crates.

use crate::error::ConfigError;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use std::fmt;
use toml;

/// Private struct used exclusively for TOML file serialization.
/// Keeps credentials in plaintext so they can be written to and read from disk.
/// Never use this for logging or network output — use `Config` directly instead.
#[derive(Serialize)]
struct ConfigToml<'a> {
    app_key: &'a str,
    api_key: &'a str,
    mac_address: &'a str,
    output_folder: &'a str,
    filename_pattern: &'a str,
    station_name: &'a str,
    tz_name: &'a str,
    detail_level: u8,
    limit: u16,
    sleep_time: u64,
}

impl<'a> From<&'a Config> for ConfigToml<'a> {
    fn from(c: &'a Config) -> Self {
        Self {
            app_key: &c.app_key,
            api_key: &c.api_key,
            mac_address: &c.mac_address,
            output_folder: &c.output_folder,
            filename_pattern: &c.filename_pattern,
            station_name: &c.station_name,
            tz_name: &c.tz_name,
            detail_level: c.detail_level,
            limit: c.limit,
            sleep_time: c.sleep_time,
        }
    }
}

/// The `Config` struct holds the configuration information for the Ambient Weather API.
#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Config {
    /// The Ambient Weather Application key.
    pub app_key: String,

    /// The Ambient Weather API key.
    pub api_key: String,

    /// The MAC address of the Ambient Weather device.
    pub mac_address: String,

    /// The folder into which the output files are to be written.
    pub output_folder: String,

    /// strftime-style pattern for output filenames. Supports `{mac}` as a
    /// placeholder for the sanitized MAC address (colons replaced with dashes),
    /// and `{station}` which resolves to `station_name` if set, or the
    /// normalized MAC address otherwise.
    /// Example: `%Y/%m/{station}-%Y-%m-%d.json`
    pub filename_pattern: String,

    /// Optional human-readable station name used as `{station}` in filename
    /// patterns. Falls back to the normalized MAC address when empty.
    pub station_name: String,

    /// Timezone in IANA format
    pub tz_name: String,

    /// The level of detail to include in the output.
    pub detail_level: u8,

    /// The number of records to limit the output to.
    pub limit: u16,

    /// Sleep duration in seconds.
    pub sleep_time: u64,
}

impl Config {
    /// Create a new, empty `Config` struct with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Read a `Config` struct from a TOML file.
    ///
    /// # Arguments
    ///
    /// * `filename` - The name of the file to read the configuration from.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Config` struct if successful, or an error if not.
    ///
    /// # Errors
    ///
    /// If the file cannot be read or the configuration cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```
    /// use shared::config::Config;
    /// let config = Config::from_file("ambient_download.toml").expect("Unable to read configuration file.");
    /// ```
    pub fn from_file(filename: &str) -> Result<Self, ConfigError> {
        let config_str = std::fs::read_to_string(filename).map_err(|e| ConfigError::Read {
            path: filename.to_string(),
            source: e,
        })?;
        let config: Self = toml::from_str(&config_str).map_err(|e| ConfigError::Parse {
            path: filename.to_string(),
            source: e,
        })?;

        log::debug!("Read configuration from {filename}: {config:?}");

        Ok(config)
    }

    /// Read a `Config` struct from command line arguments.
    ///
    /// # Arguments
    ///
    /// * `&ArgMatches` - The Clap command line arguments to read the configuration from.
    ///
    /// # Returns
    ///
    /// A `Config` struct with the values read from the command line.
    #[must_use]
    pub fn from_args(cli_args: &ArgMatches) -> Self {
        let empty_string = String::new();

        let mut config = Self::new();
        config.api_key.clone_from(
            cli_args
                .get_one::<String>("api-key")
                .unwrap_or(&empty_string),
        );
        config.app_key.clone_from(
            cli_args
                .get_one::<String>("app-key")
                .unwrap_or(&empty_string),
        );

        if let Some(weather_args) = cli_args.subcommand_matches("weather") {
            config.mac_address.clone_from(
                weather_args
                    .get_one::<String>("mac-address")
                    .unwrap_or(&empty_string),
            );
        } else {
            config.mac_address.clone_from(&empty_string);
        }

        config.output_folder.clone_from(
            cli_args
                .get_one::<String>("output-folder")
                .unwrap_or(&empty_string),
        );

        if let Some(weather_args) = cli_args.subcommand_matches("weather") {
            config.tz_name.clone_from(
                weather_args
                    .get_one::<String>("tz-name")
                    .unwrap_or(&empty_string),
            );
            config.limit = *weather_args.get_one::<u16>("limit").unwrap_or(&288);
            let default_filename_pattern = String::from("%Y-%m-%d.json");
            config.filename_pattern.clone_from(
                weather_args
                    .get_one::<String>("filename-pattern")
                    .unwrap_or(&default_filename_pattern),
            );
            config.station_name.clone_from(
                weather_args
                    .get_one::<String>("station-name")
                    .unwrap_or(&String::new()),
            );
        } else {
            config.tz_name = iana_time_zone::get_timezone().unwrap_or_default();
            config.limit = 288;
            config.filename_pattern = String::from("%Y-%m-%d.json");
            config.station_name = String::new();
        }

        if let Some(weather_args) = cli_args.subcommand_matches("weather") {
            config.sleep_time = *weather_args.get_one::<u64>("sleep-time").unwrap_or(&10);
        } else {
            config.sleep_time = 10;
        }

        config.detail_level = *cli_args.get_one::<u8>("detail").unwrap_or(&1);

        log::debug!("Read configuration from CLI: {config:?}");

        config
    }

    /// Write a `Config` struct to a TOML file. If the file doesn't exist, create a new one.
    ///
    /// # Arguments
    ///
    /// * `filename` - The name of the file to write the configuration to.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` if successful, or an error if not.
    ///
    /// # Errors
    ///
    /// If the file cannot be written or the configuration cannot be serialized.
    ///
    /// # Examples
    ///
    /// ```
    /// use shared::config::Config;
    /// let config = Config::new();
    /// config.to_file("ambient_download.toml").expect("Unable to write configuration file.");
    /// ```
    pub fn to_file(&self, filename: &str) -> Result<(), ConfigError> {
        if !std::path::Path::new(&filename).exists() {
            Self::new_config_file(filename)?;
            log::debug!("Created new configuration file {filename}");
        }

        let config_str = toml::to_string(&ConfigToml::from(self))
            .map_err(|e| ConfigError::Serialize(e.to_string()))?;
        std::fs::write(filename, &config_str).map_err(|e| ConfigError::Write {
            path: filename.to_string(),
            source: e,
        })?;

        log::debug!("Wrote configuration to {filename}.");
        log::warn!(
            "'{filename}' contains API credentials in plaintext. \
             Restrict its permissions (chmod 600) and exclude it from version control."
        );

        Ok(())
    }

    /// Create a new configuration file with default values.
    ///
    /// # Arguments
    ///
    /// * `filename` - The name of the file to create.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` if successful, or an error if not.
    ///
    /// # Errors
    ///
    /// If the file cannot be written or the configuration cannot be serialized.
    ///
    /// # Examples
    ///
    /// ```
    /// use shared::config::Config;
    /// Config::new_config_file("ambient_download.toml").expect("Unable to create configuration file.");
    /// ```
    pub fn new_config_file(filename: &str) -> Result<(), ConfigError> {
        let config = Self::new();

        let config_str = toml::to_string(&ConfigToml::from(&config))
            .map_err(|e| ConfigError::Serialize(e.to_string()))?;
        std::fs::write(filename, config_str).map_err(|e| ConfigError::Write {
            path: filename.to_string(),
            source: e,
        })?;

        Ok(())
    }
}

/// Custom `Serialize` impl that redacts `api_key` and `app_key`.
/// This makes accidental serialization (e.g. to JSON for logging) safe.
/// File I/O uses the private `ConfigToml` struct directly to preserve real values.
impl serde::Serialize for Config {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("Config", 10)?;
        s.serialize_field("app_key", "[REDACTED]")?;
        s.serialize_field("api_key", "[REDACTED]")?;
        s.serialize_field("mac_address", &self.mac_address)?;
        s.serialize_field("output_folder", &self.output_folder)?;
        s.serialize_field("filename_pattern", &self.filename_pattern)?;
        s.serialize_field("station_name", &self.station_name)?;
        s.serialize_field("tz_name", &self.tz_name)?;
        s.serialize_field("detail_level", &self.detail_level)?;
        s.serialize_field("limit", &self.limit)?;
        s.serialize_field("sleep_time", &self.sleep_time)?;
        s.end()
    }
}

/// Custom `Debug` impl that redacts sensitive credential fields.
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("app_key", &"[REDACTED]")
            .field("api_key", &"[REDACTED]")
            .field("mac_address", &self.mac_address)
            .field("output_folder", &self.output_folder)
            .field("filename_pattern", &self.filename_pattern)
            .field("station_name", &self.station_name)
            .field("tz_name", &self.tz_name)
            .field("detail_level", &self.detail_level)
            .field("limit", &self.limit)
            .field("sleep_time", &self.sleep_time)
            .finish()
    }
}

/// Implement the `Default` trait for the `Config` struct.
impl Default for Config {
    /// Create a new, empty `Config` struct with default values.
    fn default() -> Self {
        Self {
            app_key: String::new(),
            api_key: String::new(),
            mac_address: String::new(),
            output_folder: String::from("."),
            filename_pattern: String::from("%Y-%m-%d.json"),
            station_name: String::new(),
            tz_name: iana_time_zone::get_timezone().unwrap_or_default(),
            detail_level: 1,
            limit: 288,
            sleep_time: 10,
        }
    }
}
