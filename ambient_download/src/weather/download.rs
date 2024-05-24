//! Downloads the weather information from the Ambient Weather API.

use std::io::Write;

use crate::creds::{self, Query};
use crate::detail::DetailLevel;
use chrono::{DateTime, Local, TimeZone};
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

    let mut dates = Vec::<&str>::new();
    for date in subcmd_args
        .get_many::<String>("end-dates")
        .unwrap_or_default()
    {
        dates.push(date);
    }
    log::debug!("dates = {dates:?}.");

    if dates.is_empty() {
        download_yesterdays_weather(&config.output_folder, creds, detail_level)?;
    }

    Ok(())
}

/// Download yesterday's weather information and write it to a file.
///
/// # Arguments
///
/// * `detail_level` - The level of detail to log.
/// * `creds` - The credentials to use.
/// * `output_folder` - The folder to write the output file to.
///
/// # Returns
///
/// A `Result` with the number of bytes written to the file.
///
/// # Errors
///
/// If the output folder cannot be created, the weather information cannot be downloaded, or the file cannot be written.
pub fn download_yesterdays_weather(
    output_folder: &str,
    creds: Query,
    detail_level: DetailLevel,
) -> Result<(), Box<dyn Error>> {
    if detail_level > DetailLevel::Quiet {
        log::info!("No dates provided. Getting yesterday's weather information.");
    }

    let yesterday_eod = end_of_day(&yesterday())?;
    let weather_data = download_weather(&yesterday_eod, &creds)?;

    let output_file_name = filename_from_datetime(&yesterday_eod, "json");
    let full_path = format!("{output_folder}/{output_file_name}");
    let bytes_written = write_weather_info_to_file(&full_path, &weather_data)?;

    if detail_level > DetailLevel::Quiet {
        log::info!("Wrote {bytes_written} bytes to {full_path}.");
    };
    Ok(())
} // fn run()

/// Get yesterday's weather information.
#[allow(clippy::module_name_repetitions)]
pub fn download_weather(date: &DateTime<Local>, creds: &Query) -> Result<String, Box<dyn Error>> {
    let limit = creds.limit.unwrap_or(288); // 288 is the maximum number of records that can be downloaded.

    let end_date_str = date.to_rfc3339();

    let Some(mac_address) = &creds.mac_address else {
        return Err("MAC address not provided.".into());
    };

    let url = format!(
        "https://rt.ambientweather.net/v1/devices/{mac_address}?apiKey={}&applicationKey={}&endDate={end_date_str}&limit={limit}",
        creds.api_key, creds.app_key
    );
    log::debug!("url = {url}");

    let resp = reqwest::blocking::get(&url)?;
    log::debug!("resp = {resp:?}");

    let res = resp.text()?;
    log::debug!("res = {res:?}");

    Ok(res)
}

/// Write the information to a JSON file
pub fn write_weather_info_to_file(
    filename: &str,
    weather_info: &str,
) -> Result<usize, std::io::Error> {
    let mut file = std::fs::File::create(filename)?;

    let res = file.write(weather_info.as_bytes())?;
    log::debug!("Wrote {res} bytes to {filename}.");

    Ok(res)
}

/// Get yesterday's date.
pub fn yesterday() -> DateTime<Local> {
    Local::now() - chrono::Duration::days(1)
}

/// Return a date at the end of the day/
pub fn end_of_day(date: &DateTime<Local>) -> Result<DateTime<Local>, Box<dyn Error>> {
    // Find the end of the day
    let Some(eod_native) = date.naive_local().date().and_hms_opt(23, 59, 30) else {
        return Err("Could not find the end of the day.".into());
    };

    let Some(eod_local) = Local.from_local_datetime(&eod_native).single() else {
        return Err("Could not convert NaiveDateTime to DateTime<Local>.".into());
    };

    log::debug!("end_of_day = {eod_local}");
    Ok(eod_local)
}

/// Return a date as a string in the format "YYYY-MM-DD".
pub fn date_to_sting(date: &DateTime<Local>) -> String {
    let date_str = date.format("%Y-%m-%d").to_string();
    log::debug!("date_str = {date_str}");

    date_str
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
    let date_str = date_to_sting(date);
    let filename = format!("{date_str}.{ext}");
    log::debug!("filename = {filename}");

    filename
}
