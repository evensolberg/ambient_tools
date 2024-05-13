use chrono::{self, Offset};
use clap::Parser;
use shared::config::Config;

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
        // Update the TOML file with the offset information.
        // If there is no file, create a new one with the offset information.
        let mut config = Config::from_file(&config_file).unwrap_or_default();
        config.tz_offset = tz_offset;
        config
            .to_file(&config_file)
            .expect("Unable to write configuration file.");
    }
}
