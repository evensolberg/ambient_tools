use chrono::{self, Offset};
use clap::Parser;

use serde::{Deserialize, Serialize};

/// The configuration struct that will be read from and written to a TOML file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Config {
    app_key: String,
    api_key: String,
    mac_address: String,
    output_folder: String,
    tz_offset: String,
    detail_level: u8,
    limit: u16,
}

/// Output the local and UTC time, the local offset, the local timezone offset, and the local timezone name.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// The configuration file to write the offset information into.
    #[clap(short, long)]
    config_file: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let local_time = chrono::Local::now();
    println!("Local time: {local_time}");

    let utc_time = chrono::Utc::now();
    println!("UTC time:   {utc_time}\n");

    let local_offset = local_time.offset().fix().local_minus_utc();
    println!("Local offset:          {local_offset} seconds");

    let tz_offset = local_time.offset().to_string();
    println!("Local timezone offset: {tz_offset} (HH:MM)");

    if let Ok(tz_name) = iana_time_zone::get_timezone() {
        println!("Local timezone name:   {tz_name}");
    } else {
        println!("Unable to determine local timezone name.");
    }

    if let Some(config_file) = cli.config_file {
        // Check if the configuration file exists. If it doesn't, create a new one.
        if !std::path::Path::new(&config_file).exists() {
            new_config_file(&config_file);
        }

        // Read the toml file and update the offset information. Then write the updated information back to the file.
        let config_str =
            std::fs::read_to_string(&config_file).expect("Unable to read configuration file.");
        let mut config: Config =
            toml::from_str(&config_str).expect("Unable to parse configuration file.");

        config.tz_offset = tz_offset;

        let updated_config_str =
            toml::to_string(&config).expect("Unable to serialize configuration file.");
        std::fs::write(config_file, updated_config_str)
            .expect("Unable to write configuration file.");
    }
}

fn new_config_file(filename: &str) {
    let config = Config {
        app_key: String::new(),
        api_key: String::new(),
        mac_address: String::new(),
        output_folder: String::from("weather_data"),
        tz_offset: String::new(),
        detail_level: 1,
        limit: 288,
    };

    let config_str = toml::to_string(&config).expect("Unable to serialize configuration file.");
    std::fs::write(filename, config_str).expect("Unable to write configuration file.");
}
