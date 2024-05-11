use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub app_key: String,
    pub api_key: String,
    pub mac_address: String,
    pub output_folder: String,
    pub tz_offset: String,
    pub detail_level: u8,
    pub limit: u16,
}

impl Config {
    /// Create a new, empty `Config` struct with default values.
    pub fn new() -> Self {
        Self {
            api_key: String::new(),
            app_key: String::new(),
            mac_address: String::new(),
            output_folder: String::new(),
            tz_offset: String::new(),
            detail_level: 0,
            limit: 0,
        }
    }

    pub fn from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = std::fs::read_to_string(filename)?;
        let config: Config = toml::from_str(&config_str)?;

        log::debug!("Read configuration from {filename}: {config:?}");

        Ok(config)
    }

    pub fn from_args(cli_args: &ArgMatches) -> Self {
        let empty_string = String::new();
        let no_offset = String::from("+00:00");

        let mut config = Config::new();
        config.api_key = cli_args
            .get_one::<String>("api-key")
            .unwrap_or(&empty_string)
            .to_string();
        config.app_key = cli_args
            .get_one::<String>("app-key")
            .unwrap_or(&empty_string)
            .to_string();
        config.mac_address = cli_args
            .get_one::<String>("mac-address")
            .unwrap_or(&empty_string)
            .to_string();
        config.output_folder = cli_args
            .get_one::<String>("mac-address")
            .unwrap_or(&empty_string)
            .to_string();

        if let Some(weather_args) = cli_args.subcommand_matches("weather") {
            config.tz_offset = weather_args
                .get_one::<String>("tz-offset")
                .unwrap_or(&no_offset)
                .to_string();
        } else {
            config.tz_offset = no_offset;
        }

        config.detail_level = *cli_args.get_one::<u8>("detail-level").unwrap_or(&1);
        config.limit = *cli_args.get_one::<u16>("limit").unwrap_or(&288);

        log::debug!("Read configuration from CLI: {config:?}");

        config
    }
}
