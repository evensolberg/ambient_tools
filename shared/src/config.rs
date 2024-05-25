//! Implement the `Config` struct and related functions. This is used in the `ambient_download` and `tz_detect` crates.

use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use toml;

/// The `Config` struct holds the configuration information for the Ambient Weather API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// The Ambient Weather Application key.
    pub app_key: String,

    /// The Ambient Weather API key.
    pub api_key: String,

    /// The MAC address of the Ambient Weather device.
    pub mac_address: String,

    /// The folder into which the output files are to be written.
    pub output_folder: String,

    /// The timezone offset.
    pub tz_offset: String,

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
    pub fn from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = std::fs::read_to_string(filename)?;
        let config: Self = toml::from_str(&config_str)?;

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
        let no_offset = String::from("+00:00");

        let mut config = Self::new();
        config.api_key = cli_args
            .get_one::<String>("api-key")
            .unwrap_or(&empty_string)
            .to_string();
        config.app_key = cli_args
            .get_one::<String>("app-key")
            .unwrap_or(&empty_string)
            .to_string();

        if let Some(weather_args) = cli_args.subcommand_matches("weather") {
            config.mac_address = weather_args
                .get_one::<String>("mac-address")
                .unwrap_or(&empty_string)
                .to_string();
        } else {
            config.mac_address.clone_from(&empty_string);
        }

        config.output_folder = cli_args
            .get_one::<String>("output-folder")
            .unwrap_or(&empty_string)
            .to_string();

        if let Some(weather_args) = cli_args.subcommand_matches("weather") {
            config.tz_offset = weather_args
                .get_one::<String>("tz-offset")
                .unwrap_or(&no_offset)
                .to_string();
            config.limit = *weather_args.get_one::<u16>("limit").unwrap_or(&288);
        } else {
            config.tz_offset = no_offset;
            config.limit = 288;
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
    pub fn to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !std::path::Path::new(&filename).exists() {
            Self::new_config_file(filename)?;
            log::debug!("Created new configuration file {filename}");
        }

        let config_str = toml::to_string(self)?;
        std::fs::write(filename, &config_str)?;

        log::debug!("Wrote configuration to {filename}: {config_str:?}");

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
    pub fn new_config_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config = Self::new();

        let config_str = toml::to_string(&config)?;
        std::fs::write(filename, config_str)?;

        Ok(())
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
            tz_offset: String::from("+00:00"),
            detail_level: 1,
            limit: 288,
            sleep_time: 10,
        }
    }
}
