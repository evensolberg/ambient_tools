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
pub fn get_weather(
    cli_args: &clap::ArgMatches,
    config: config::Config,
    creds: creds::Query,
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
        .get_one::<String>("end-dates")
        .unwrap_or(&empty_str);
    let num_days = *subcmd_args.get_one::<u16>("days").unwrap_or(&1);

    let dl_date = if date.is_empty() {
        yesterday()
    } else {
        let parse_date = format!("{date} 00:00:30 {}", config.tz_offset);

        let Ok(parsed_date) = DateTime::parse_from_str(&parse_date, "%F %T %:z") else {
            let err_msg = format!("Could not parse {parse_date}.");
            return Err(err_msg.into());
        };

        parsed_date.with_timezone(&Local)
    };

    download_weather(
        &end_of_day(&dl_date)?,
        num_days,
        &creds,
        &config,
        detail_level,
    )?;

    Ok(())
}

/// Get yesterday's weather information.
#[allow(clippy::module_name_repetitions)]
pub fn download_weather_data(
    date: &DateTime<Local>,
    creds: &Query,
) -> Result<String, Box<dyn Error>> {
    let limit = creds.limit.unwrap_or(288); // 288 is the maximum number of records that can be downloaded.
    let end_date_str = date.to_rfc3339();

    let Some(mac_address) = &creds.mac_address else {
        return Err("MAC address not provided.".into());
    };

    let url = format!(
        "https://rt.ambientweather.net/v1/devices/{mac_address}?apiKey={}&applicationKey={}&endDate={end_date_str}&limit={limit}",
        creds.api_key, creds.app_key
    );
    let resp = reqwest::blocking::get(url)?.text()?;

    Ok(resp)
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

/// Download the weather information for a number of days, ending with the date specified
fn download_weather(
    end_date: &DateTime<Local>,
    num_days: u16,
    creds: &Query,
    config: &config::Config,
    detail_level: DetailLevel,
) -> Result<(), Box<dyn Error>> {
    if detail_level > DetailLevel::Quiet {
        log::info!("Getting weather information for {num_days} days ending at {end_date}.");
    }

    if num_days > 365 * 3 {
        return Err("The maximum number of days is 1095 (3 years).".into());
    }

    let sleep_duration = 10; // How long to sleep between downloads to avoid rate limiting.
    let mut date = *end_date;
    let output_folder = &config.output_folder;

    for d in 0..num_days {
        let weather_info = download_weather_data(&date, creds)?;

        let output_file_name = filename_from_datetime(&date, "json");
        let full_path = format!("{output_folder}/{output_file_name}");
        let bytes_written = write_weather_info_to_file(&full_path, &weather_info)?;

        if detail_level > DetailLevel::Quiet {
            log::info!("Wrote {bytes_written} bytes to {full_path}.");
        };

        date -= chrono::Duration::days(1);

        if num_days > 1 && d < num_days - 1 {
            if detail_level > DetailLevel::Quiet {
                log::info!("Sleeping for {sleep_duration} seconds to avoid rate limiting.");
            }
            sleep(std::time::Duration::from_secs(sleep_duration));
        }
    }

    Ok(())
}

/// Get yesterday's date.
pub fn yesterday() -> DateTime<Local> {
    Local::now() - chrono::Duration::days(1)
}

/// Return a date at the end of the day/
pub fn end_of_day(date: &DateTime<Local>) -> Result<DateTime<Local>, Box<dyn Error>> {
    // Find the end of the day
    let eod_local = date
        .with_time(NaiveTime::from_hms_opt(23, 59, 30).unwrap_or_default())
        .unwrap();

    Ok(eod_local)
}
