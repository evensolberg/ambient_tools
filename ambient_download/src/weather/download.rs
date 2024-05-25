//! Downloads the weather information from the Ambient Weather API.

use std::io::Write;
use std::thread::sleep;

use crate::creds::{self, Query};
use crate::detail::DetailLevel;
use chrono::{DateTime, Local, NaiveTime};
use shared::config;
use std::error::Error;

/// Get the weather information and write it to a file.
///
/// # Arguments
///
/// * `cli_args` - The command line arguments.
/// * `config` - The configuration.
/// * `detail_level` - The level of detail to log.
/// * `creds` - The credentials to use.
///
/// # Returns
///
/// An empty `Ok` Result if successful.
///
/// # Errors
///
/// If the output folder cannot be created.
/// If the weather information cannot be downloaded.
/// If the file cannot be written.
///
/// # Panics
///
/// If the weather subcommand is not found.
pub fn get_weather_data(
    cli_args: &clap::ArgMatches,
    config: &config::Config,
    creds: &creds::Query,
    detail_level: DetailLevel,
) -> Result<(), Box<dyn Error>> {
    if detail_level > DetailLevel::Quiet {
        log::info!("Getting weather information.");
    }

    crate::check_or_create_output_folder(&config.output_folder)?;

    let subcmd_args = cli_args
        .subcommand_matches("weather")
        .expect("Weather subcommand not found. Yikes!");

    let empty_str = String::new();
    let date = subcmd_args
        .get_one::<String>("start-date")
        .unwrap_or(&empty_str);
    let num_days = *subcmd_args.get_one::<u16>("days").unwrap_or(&1);

    let dl_date = if date.is_empty() {
        end_of_day(&yesterday())
    } else {
        let date_to_parse = format!("{date} 23:59:30 {}", config.tz_offset);

        let Ok(parsed_date) = DateTime::parse_from_str(&date_to_parse, "%F %T %:z") else {
            let err_msg = format!("Could not parse {date_to_parse}.");
            return Err(err_msg.into());
        };

        parsed_date.with_timezone(&Local)
    };

    download_weather(&dl_date, num_days, creds, config, detail_level)?;

    Ok(())
}

/// Write the information to a JSON file
pub fn write_weather_info_to_file(
    filename: &str,
    weather_info: &str,
) -> Result<usize, std::io::Error> {
    let mut file = std::fs::File::create(filename)?;
    let res = file.write(weather_info.as_bytes())?;

    Ok(res)
}

/// Return a filename in the format "YYYY-MM-DD.ext" from a date.
///
/// # Arguments
///
/// * `date` - The date to use in the filename.
/// * `ext` - The extension to use in the filename.
///
/// # Returns
///
/// A `Result` containing the filename if successful, or an error if not.
///
/// # Errors
///
/// If the date cannot be converted to a string.
///
/// # Examples
///
/// ```
/// let date = Local::now();
/// let filename = filename_from_datetime(&date, "json").expect("Could not create filename.");
/// ```
pub fn filename_from_datetime(date: &DateTime<Local>, ext: &str) -> String {
    let date_str = date.format("%Y-%m-%d").to_string();
    let filename = format!("{date_str}.{ext}");

    filename
}

/// Download the weather information for a number of days, starting with the date specified
fn download_weather(
    start_date: &DateTime<Local>,
    num_days: u16,
    creds: &Query,
    config: &config::Config,
    detail_level: DetailLevel,
) -> Result<(), Box<dyn Error>> {
    if detail_level > DetailLevel::Quiet {
        log::info!("Getting weather information for {num_days} days starting at {start_date}.");
    }

    if num_days > 365 * 3 {
        return Err("The maximum number of days is 1095 (3 years).".into());
    }

    let mut date = *start_date;
    let output_folder = &config.output_folder;

    let limit = creds.limit.unwrap_or(288); // 288 is the maximum number of records that can be downloaded.

    // Create a reqwest blocking client that will be used to download the weather information.
    let timeout = std::time::Duration::from_secs(&config.sleep_time + 5);
    let kas = if config.sleep_time > 20 { 10 } else { 5 };
    let keepalive = std::time::Duration::from_secs(kas);
    let client = reqwest::blocking::Client::builder()
        .timeout(Some(timeout))
        .tcp_keepalive(keepalive)
        .user_agent("ambient-downloader")
        .build()?;

    if detail_level > DetailLevel::Quiet {
        log::info!(
            "Waiting for {} seconds between each download to avoid overloading servers and being hit by rate limiting.",
            config.sleep_time
        );
    }

    for d in 0..num_days {
        let Some(mac_address) = &creds.mac_address else {
            return Err("MAC address not provided.".into());
        };

        let url = format!(
            "https://rt.ambientweather.net/v1/devices/{mac_address}?apiKey={}&applicationKey={}&endDate={}&limit={limit}",
            creds.api_key, creds.app_key, date.to_rfc3339()
        );

        let res = client.get(url).send();
        let weather_info;
        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    weather_info = resp.text()?;
                } else {
                    let err_msg = format!(
                        "Error downloading weather information. Status code: {}.",
                        resp.status().as_str()
                    );
                    return Err(err_msg.into());
                }
            }
            Err(e) => {
                let err_msg = format!("Error downloading weather information: {e}.");
                return Err(err_msg.into());
            }
        }

        let output_file_name = filename_from_datetime(&date, "json");
        let full_path = format!("{output_folder}/{output_file_name}");
        let bytes_written = write_weather_info_to_file(&full_path, &weather_info)?;

        if detail_level > DetailLevel::Quiet {
            log::info!("Wrote {bytes_written} bytes to {full_path}.");
        };

        date += chrono::Duration::days(1);

        if num_days > 1 && d < num_days - 1 {
            sleep(std::time::Duration::from_secs(config.sleep_time));
        }
    }

    Ok(())
}

/// Get yesterday's date.
pub fn yesterday() -> DateTime<Local> {
    Local::now() - chrono::Duration::days(1)
}

/// Return a date at the end of the day/
pub fn end_of_day(date: &DateTime<Local>) -> DateTime<Local> {
    // Find the end of the day
    date.with_time(NaiveTime::from_hms_opt(23, 59, 30).unwrap_or_default())
        .unwrap()
}
