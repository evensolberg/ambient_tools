//! Contains the `Credentials` struct and its implementation.

use chrono::{DateTime, FixedOffset, Local};
use std::{error::Error, fmt, str::FromStr};

/// Returns true if `key` is a valid 64-character lowercase hex string.
fn is_valid_api_key(key: &str) -> bool {
    key.len() == 64 && key.chars().all(|c| c.is_ascii_hexdigit())
}

/// Returns true if `mac` is a valid MAC address.
/// Accepts `XX:XX:XX:XX:XX:XX`, `XX-XX-XX-XX-XX-XX`, or `XXXXXXXXXXXX`.
fn is_valid_mac(mac: &str) -> bool {
    let digits: String = mac.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    if digits.len() != 12 {
        return false;
    }
    let sep_count = mac.chars().filter(|&c| c == ':' || c == '-').count();
    sep_count == 0 || sep_count == 5
}

use crate::config;
use crate::query::{self, QueryType};

/// A struct to hold the credentials needed to authenticate with the Ambient Weater API and
/// the MAC address of the console for which we are downloading information.
#[allow(clippy::struct_field_names)]
#[derive(Clone, PartialEq, Eq)]
pub struct Query {
    /// The type of query we are performing.
    pub query_type: query::QueryType,

    /// The API Key for the Ambient Weather API. This is needed for all requests and can be
    /// obtained from the Ambient Weather website.
    pub api_key: String,

    /// The Application Key for the Ambient Weather API. This is needed for all requests and can be
    /// obtained from the Ambient Weather website.
    pub app_key: String,

    /// The MAC address of the console for which we are downloading information. This is only
    /// needed when downloading weather information. Format: `XX:XX:XX:XX:XX:XX`.
    ///
    /// You can find the MAC address on the back of the console, in Ambient Weather's app, or simply by
    /// downloading the device information using the `device` subcommand of this program.
    pub mac_address: Option<String>,

    /// The end date for the query. This is basically the date for which we are downloading.
    /// This is only needed when downloading weather information. Format: `YYYY-MM-DD`.
    ///
    /// If not provided, the program will download yesterday's weather information.
    pub end_date: Option<Vec<DateTime<Local>>>,

    /// The number of records to download. This is only needed when downloading weather information.
    /// The maximum number of records that can be downloaded is 288. If not provided, the program
    /// will download the maximum number of records.
    pub limit: Option<u16>,

    /// The timezone offset. This is only needed when downloading weather information. Format: `+HH:MM`.
    /// If not provided, the program will use the local timezone.
    pub tz_offset: Option<FixedOffset>,
}

impl Query {
    /// Create a new, empty `Credentials` struct with default values.
    #[allow(dead_code)]
    fn new() -> Self {
        Self::default()
    }

    /// Create a new `Credentials` struct.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The Ambient Weather API key.
    /// * `app_key` - The Ambient Weather Application key.
    /// * `mac_address` - The MAC address of the console for which we are downloading information.
    ///
    /// # Returns
    ///
    /// A new `Credentials` struct.
    #[allow(dead_code, clippy::similar_names)]
    pub fn from(query_type: QueryType, api_key: &str, app_key: &str) -> Self {
        Self {
            query_type,
            api_key: api_key.to_string(),
            app_key: app_key.to_string(),
            ..Default::default()
        }
    }

    /// Create a new `Credentials` struct from the environment variables.
    ///
    /// # Returns
    ///
    /// A new `Credentials` struct.
    ///
    /// # Panics
    ///
    /// If the environment variables are not set.
    ///
    /// # Errors
    ///
    /// If the environment variables are not set.
    ///
    /// # Example
    /// ```
    /// let creds = Credentials::from_env().unwrap();
    /// ```
    #[allow(dead_code)]
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            api_key: std::env::var("AMBIENT_WEATHER_API_KEY")?,
            app_key: std::env::var("AMBIENT_WEATHER_APP_KEY")?,
            mac_address: std::env::var("AMBIENT_WEATHER_MAC_ADDRESS").ok(),
            tz_offset: std::env::var("AMBIENT_WEATHER_TZ_OFFSET")
                .map_or(None, |f| FixedOffset::from_str(&f).ok()),
            ..Default::default()
        })
    }

    /// Create a new `Credentials` struct from the command line arguments.
    ///
    /// # Arguments
    ///
    /// * `cli_args` - The command line arguments.
    ///
    /// # Returns
    ///
    /// A new `Credentials` struct.
    #[allow(dead_code)]
    pub fn from_cli(cli_args: &clap::ArgMatches) -> Self {
        // Match the cli_args to the appropriate QueryType and return the necessary information.
        match cli_args.subcommand_name() {
            Some("device") => Self::device_query_from_args(cli_args),
            Some("weather") => Self::weather_query_from_args(cli_args),
            _ => {
                // Default is to print the help message.
                Self {
                    query_type: QueryType::Help,
                    ..Default::default()
                }
            }
        }
    }

    /// Returns the credentials needed to download device information.
    fn device_query_from_args(cli_args: &clap::ArgMatches) -> Self {
        let empty = String::new();

        Self {
            query_type: QueryType::GetDeviceInfo,
            api_key: cli_args
                .get_one::<String>("api-key")
                .unwrap_or(&empty)
                .to_string(),
            app_key: cli_args
                .get_one::<String>("app-key")
                .unwrap_or(&empty)
                .to_string(),
            ..Default::default()
        }
    }

    /// Returns the credentials needed to download weather information.
    fn weather_query_from_args(cli_args: &clap::ArgMatches) -> Self {
        let empty = String::new();

        let weather_args = cli_args
            .subcommand_matches("weather")
            .expect("Weather subcommand not found. This is bad.");

        Self {
            query_type: QueryType::GetWeather,
            api_key: cli_args
                .get_one::<String>("api-key")
                .unwrap_or(&empty)
                .to_string(),
            app_key: cli_args
                .get_one::<String>("app-key")
                .unwrap_or(&empty)
                .to_string(),
            mac_address: weather_args
                .get_one::<String>("mac-address")
                .map(std::string::ToString::to_string),
            end_date: weather_args.get_many::<String>("end-dates").map(|dates| {
                dates
                    .map(|date| {
                        DateTime::parse_from_rfc3339(date)
                            .unwrap_or_default()
                            .with_timezone(&Local)
                    })
                    .collect()
            }),
            limit: weather_args.get_one::<u16>("limit").or(None).copied(),
            tz_offset: weather_args
                .get_one::<String>("tz-offset")
                .map(|tz| FixedOffset::from_str(tz).expect("Unable to parse timezone offset.")),
        }
    }

    /// Validate that credentials are present and well-formed before making API calls.
    ///
    /// # Errors
    ///
    /// Returns an error if `api_key` or `app_key` are missing or not 64-character hex strings,
    /// or if `mac_address` is present but not a valid MAC address.
    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        if self.api_key.is_empty() {
            return Err("API key is not set. Use --api-key or AMBIENT_WEATHER_API_KEY.".into());
        }
        if !is_valid_api_key(&self.api_key) {
            return Err("API key must be a 64-character hexadecimal string.".into());
        }
        if self.app_key.is_empty() {
            return Err(
                "Application key is not set. Use --app-key or AMBIENT_WEATHER_APP_KEY.".into(),
            );
        }
        if !is_valid_api_key(&self.app_key) {
            return Err("Application key must be a 64-character hexadecimal string.".into());
        }
        if let Some(mac) = &self.mac_address {
            if !mac.is_empty() && !is_valid_mac(mac) {
                return Err(
                    format!("MAC address '{mac}' is invalid. Expected XX:XX:XX:XX:XX:XX.").into(),
                );
            }
        }
        Ok(())
    }

    /// Get the credentials from the configuration file.
    pub fn from_config(config: &config::Config) -> Self {
        Self {
            query_type: QueryType::Help,
            api_key: config.api_key.clone(),
            app_key: config.app_key.clone(),
            mac_address: Some(config.mac_address.clone()),
            end_date: None,
            limit: Some(config.limit),
            tz_offset: None,
        }
    }
}

/// Custom `Debug` impl that redacts sensitive credential fields.
impl fmt::Debug for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Query")
            .field("query_type", &self.query_type)
            .field("api_key", &"[REDACTED]")
            .field("app_key", &"[REDACTED]")
            .field("mac_address", &self.mac_address)
            .field("end_date", &self.end_date)
            .field("limit", &self.limit)
            .field("tz_offset", &self.tz_offset)
            .finish()
    }
}

impl Default for Query {
    fn default() -> Self {
        Self {
            query_type: QueryType::Help,
            api_key: String::new(),
            app_key: String::new(),
            mac_address: None,
            end_date: None,
            limit: None,
            tz_offset: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_key() -> String {
        "a".repeat(64)
    }

    fn valid_query() -> Query {
        Query {
            api_key: valid_key(),
            app_key: valid_key(),
            mac_address: Some("AA:BB:CC:DD:EE:FF".to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn validate_accepts_valid_credentials() {
        assert!(valid_query().validate().is_ok());
    }

    #[test]
    fn validate_accepts_mac_without_separators() {
        let mut q = valid_query();
        q.mac_address = Some("AABBCCDDEEFF".to_string());
        assert!(q.validate().is_ok());
    }

    #[test]
    fn validate_accepts_mac_with_hyphens() {
        let mut q = valid_query();
        q.mac_address = Some("AA-BB-CC-DD-EE-FF".to_string());
        assert!(q.validate().is_ok());
    }

    #[test]
    fn validate_rejects_empty_api_key() {
        let mut q = valid_query();
        q.api_key = String::new();
        assert!(q.validate().is_err());
    }

    #[test]
    fn validate_rejects_short_api_key() {
        let mut q = valid_query();
        q.api_key = "abc123".to_string();
        assert!(q.validate().is_err());
    }

    #[test]
    fn validate_rejects_non_hex_api_key() {
        let mut q = valid_query();
        q.api_key = "z".repeat(64);
        assert!(q.validate().is_err());
    }

    #[test]
    fn validate_rejects_empty_app_key() {
        let mut q = valid_query();
        q.app_key = String::new();
        assert!(q.validate().is_err());
    }

    #[test]
    fn validate_rejects_invalid_mac() {
        let mut q = valid_query();
        q.mac_address = Some("ZZ:ZZ:ZZ:ZZ:ZZ:ZZ".to_string());
        assert!(q.validate().is_err());
    }

    #[test]
    fn validate_rejects_mac_wrong_length() {
        let mut q = valid_query();
        q.mac_address = Some("AA:BB:CC".to_string());
        assert!(q.validate().is_err());
    }

    #[test]
    fn validate_skips_mac_check_when_none() {
        let mut q = valid_query();
        q.mac_address = None;
        assert!(q.validate().is_ok());
    }
}
