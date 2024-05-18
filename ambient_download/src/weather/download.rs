//! Downloads the weather information from the Ambient Weather API.

use std::io::Write;

use crate::creds::Query;
use chrono;
use chrono::{DateTime, Local, TimeZone};
use std::error::Error as StdError;

/// Get yesterday's weather information.
#[allow(clippy::module_name_repetitions)]
pub async fn download_weather(
    date: &DateTime<Local>,
    creds: &Query,
) -> Result<String, Box<dyn StdError>> {
    let limit = creds.limit.unwrap_or(288); // 288 is the maximum number of records that can be downloaded.

    // let end_date = creds
    //     .end_date
    //     .clone()
    //     .unwrap_or(vec![end_of_day(&yesterdays_date)?]);
    // let now = Local::now();
    // let first = end_date.first().unwrap_or(&now);
    // let end_date_str = end_of_day(first)?.to_rfc3339();

    let end_date_str = date.to_rfc3339();

    let Some(mac_address) = &creds.mac_address else {
        return Err("MAC address not provided.".into());
    };

    let url = format!(
        "https://rt.ambientweather.net/v1/devices/{mac_address}?apiKey={}&applicationKey={}&endDate={end_date_str}&limit={limit}",
        creds.api_key, creds.app_key
    );
    log::debug!("url = {url}");

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    log::debug!("resp = {resp:?}");

    let res = resp.text().await?;
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
pub fn end_of_day(date: &DateTime<Local>) -> Result<DateTime<Local>, Box<dyn StdError>> {
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
