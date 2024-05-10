//! Downloads the weather information from the Ambient Weather API.

use std::io::Write;

use crate::creds::Query;
use chrono;
use chrono::{DateTime, Local, TimeZone};
use std::error::Error as StdError;

/// Get yesterday's weather information.
pub async fn download_yesterdays_weather(creds: &Query) -> Result<String, Box<dyn StdError>> {
    let limit = creds.limit.unwrap_or(288); // 288 is the maximum number of records that can be downloaded.
    let yesterdays_date = get_yesterday_date()?;

    let end_date = creds
        .end_date
        .clone()
        .unwrap_or(vec![end_of_day(&yesterdays_date)?]);

    let now = Local::now();
    let first = end_date.first().unwrap_or(&now);
    let end_date_str = end_of_day(first)?.to_rfc3339();

    let mac_address = match &creds.mac_address {
        Some(mac) => mac,
        None => {
            return Err("MAC address not provided.".into());
        }
    };

    let url = format!(
        "https://rt.ambientweather.net/v1/devices/{}?apiKey={}&applicationKey={}&endDate={end_date_str}&limit={limit}",
        mac_address, creds.api_key, creds.app_key
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
pub fn get_yesterday_date() -> Result<DateTime<Local>, Box<dyn StdError>> {
    Ok(Local::now() - chrono::Duration::days(1))
}

/// Return a date at the end of the day/
pub fn end_of_day(date: &DateTime<Local>) -> Result<DateTime<Local>, Box<dyn StdError>> {
    // Find the end of the day
    let eod_n = match date.naive_local().date().and_hms_opt(23, 59, 30) {
        Some(eod) => eod,
        None => {
            return Err("Could not find the end of the day.".into());
        }
    };

    let eod_l = match Local.from_local_datetime(&eod_n).single() {
        Some(eod) => eod,
        None => {
            return Err("Could not convert NaiveDateTime to DateTime<Local>.".into());
        }
    };

    log::debug!("end_of_day = {eod_l}");
    Ok(eod_l)
}

/// Return a date as a string in the format "YYYY-MM-DD".
pub fn get_date_string(date: &DateTime<Local>) -> Result<String, Box<dyn StdError>> {
    let date_str = date.format("%Y-%m-%d").to_string();
    log::debug!("date_str = {date_str}");

    Ok(date_str)
}

/// Return yesterday's date as a string in the format "YYYY-MM-DD".
pub fn get_yesterday_date_string() -> Result<String, Box<dyn StdError>> {
    let yesterday = get_yesterday_date()?;
    let date_str = get_date_string(&yesterday)?;

    Ok(date_str)
}

/// Get the filename for yesterday's weather information.
pub fn get_yesterdays_weather_filename() -> Result<String, Box<dyn StdError>> {
    let date_str = get_yesterday_date_string()?;
    let filename = format!("{date_str}.json");
    log::debug!("filename = {filename}");

    Ok(filename)
}
