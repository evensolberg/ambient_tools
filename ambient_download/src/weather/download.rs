//! Downloads the weather information from the Ambient Weather API.

use anyhow::{bail, Context, Result};
use std::io::Write;
use std::thread::sleep;

use crate::creds::{self, Query};
use crate::detail::DetailLevel;
use chrono::TimeZone;
use chrono::{DateTime, Local, NaiveTime};
use chrono_tz::OffsetComponents;
use clap::parser::ValueSource;

use shared::config;

/// Returns the recommended minimum sleep time in seconds for a given number of days.
/// Scales up to avoid rate limiting on larger downloads.
const fn recommended_sleep_for_days(num_days: u16) -> u64 {
    match num_days {
        0..=5 => 10,
        6..=15 => 30,
        16..=30 => 60,
        31..=90 => 120,
        _ => 300,
    }
}

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
) -> Result<()> {
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
        let tz_offset = get_offset_from_tz(&config.tz_name)?;
        let date_to_parse = format!("{date} 23:59:30 {tz_offset}",);

        let Ok(parsed_date) = DateTime::parse_from_str(&date_to_parse, "%F %T %:z") else {
            bail!("Could not parse date '{date_to_parse}'.");
        };

        parsed_date.with_timezone(&Local)
    };

    // Auto-scale sleep time when the user left it at the default.
    let sleep_time = if subcmd_args.value_source("sleep-time") == Some(ValueSource::DefaultValue) {
        let recommended = recommended_sleep_for_days(num_days);
        if recommended != config.sleep_time && detail_level > DetailLevel::Quiet {
            log::info!(
                "Auto-scaling sleep time to {recommended}s for {num_days} days. \
                 Use --sleep-time to override."
            );
        }
        recommended
    } else {
        config.sleep_time
    };

    download_weather(&dl_date, num_days, creds, config, sleep_time, detail_level)?;

    Ok(())
}

/// Write the information to a JSON file, pretty-printed for readability.
///
/// # Arguments
///
/// * `filename` - The name of the file to write to.
/// * `weather_info` - The weather information to write to the file.
///
/// # Returns
///
/// A `Result` containing the number of bytes written to the file if successful, or an error if not.
///
/// # Errors
///
/// If the file cannot be created or written.
/// If the JSON cannot be parsed or serialized.
pub fn write_weather_info_to_file(filename: &str, weather_info: &str) -> Result<usize> {
    let value: serde_json::Value = serde_json::from_str(weather_info)
        .with_context(|| format!("Failed to parse JSON for {filename}"))?;
    let pretty = serde_json::to_string_pretty(&value)
        .with_context(|| format!("Failed to serialize JSON for {filename}"))?;

    let mut file = std::fs::File::create(filename)?;
    let res = file.write(pretty.as_bytes())?;

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
    sleep_time: u64,
    detail_level: DetailLevel,
) -> Result<()> {
    if detail_level > DetailLevel::Quiet {
        log::info!("Getting weather information for {num_days} days starting at {start_date}.");
    }

    if num_days > 365 * 3 {
        bail!("The maximum number of days is 1095 (3 years).");
    }

    let mut date = *start_date;
    let output_folder = &config.output_folder;

    let limit = creds.limit.unwrap_or(288); // 288 is the maximum number of records that can be downloaded.

    // Create a reqwest blocking client that will be used to download the weather information.
    let timeout = std::time::Duration::from_secs(sleep_time + 5);
    let kas = if sleep_time > 20 { 10 } else { 5 };
    let keepalive_secs = std::time::Duration::from_secs(kas);
    let client = reqwest::blocking::Client::builder()
        .timeout(Some(timeout))
        .tcp_keepalive(keepalive_secs)
        .user_agent("ambient-downloader")
        .build()?;

    if detail_level > DetailLevel::Quiet {
        log::info!("Waiting for {sleep_time}s between downloads to avoid rate limiting.",);
    }

    for d in 0..num_days {
        let Some(mac_address) = &creds.mac_address else {
            bail!("MAC address not provided.");
        };

        let url = format!(
            "https://rt.ambientweather.net/v1/devices/{mac_address}?apiKey={}&applicationKey={}&endDate={}&limit={limit}",
            creds.api_key, creds.app_key, date.to_rfc3339()
        );

        let resp = client
            .get(url)
            .send()
            .context("Failed to connect to Ambient Weather API")?;

        if !resp.status().is_success() {
            bail!("API returned error status: {}", resp.status());
        }

        let weather_info = resp.text().context("Failed to read weather response")?;

        let full_path = format!("{output_folder}/{}", filename_from_datetime(&date, "json"));
        let bytes_written = write_weather_info_to_file(&full_path, &weather_info)?;

        if detail_level > DetailLevel::Quiet {
            log::info!("Wrote {bytes_written} bytes to {full_path}.");
        }

        date += chrono::Duration::days(1);

        if num_days > 1 && d < num_days - 1 {
            sleep(std::time::Duration::from_secs(sleep_time));
        }
    }

    Ok(())
}

/// Get yesterday's date.
pub fn yesterday() -> DateTime<Local> {
    Local::now() - chrono::Duration::days(1)
}

/// Return a date at the end of the day.
pub fn end_of_day(date: &DateTime<Local>) -> DateTime<Local> {
    // SAFETY: 23, 59, 30 are valid time components; from_hms_opt never returns None here.
    let t = NaiveTime::from_hms_opt(23, 59, 30).unwrap_or_default();
    // with_time returns LocalResult which can be ambiguous during DST fall-back transitions.
    // We take the earlier occurrence; if the time is skipped (DST spring-forward gap), we
    // fall back to adding seconds directly to avoid a panic.
    match date.with_time(t) {
        chrono::LocalResult::Single(dt) => dt,
        chrono::LocalResult::Ambiguous(early, _late) => early,
        chrono::LocalResult::None => {
            *date
                + chrono::Duration::hours(23)
                + chrono::Duration::minutes(59)
                + chrono::Duration::seconds(30)
        }
    }
}

/// Calculate the offset from UTC for a given timezone based on the IANA timezone input as a string
fn get_offset_from_tz(tz_name: &str) -> Result<String> {
    let tz: chrono_tz::Tz = tz_name
        .parse()
        .with_context(|| format!("Unknown timezone '{tz_name}'"))?;
    let local_time = chrono::Local::now();
    let tz_offset = tz.offset_from_utc_datetime(&local_time.naive_utc());
    let offset = tz_offset.base_utc_offset() + tz_offset.dst_offset();
    let offset_secs = offset.num_seconds();
    let offset_hrs = offset_secs / 3600;
    let offset_mins = (offset_secs % 3600) / 60;

    Ok(format!("{offset_hrs:>+03}:{offset_mins:02}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Timelike};

    #[test]
    fn filename_from_datetime_formats_correctly() {
        let dt = chrono::Local
            .with_ymd_and_hms(2024, 5, 1, 12, 0, 0)
            .unwrap();
        assert_eq!(filename_from_datetime(&dt, "json"), "2024-05-01.json");
    }

    #[test]
    fn yesterday_is_one_day_before_today() {
        let today = chrono::Local::now().date_naive();
        let y = yesterday().date_naive();
        assert_eq!((today - y).num_days(), 1);
    }

    #[test]
    fn end_of_day_sets_time_to_235930() {
        let dt = chrono::Local.with_ymd_and_hms(2024, 5, 1, 8, 0, 0).unwrap();
        let eod = end_of_day(&dt);
        assert_eq!(eod.hour(), 23);
        assert_eq!(eod.minute(), 59);
        assert_eq!(eod.second(), 30);
    }

    #[test]
    fn recommended_sleep_scales_with_days() {
        assert_eq!(recommended_sleep_for_days(1), 10);
        assert_eq!(recommended_sleep_for_days(5), 10);
        assert_eq!(recommended_sleep_for_days(6), 30);
        assert_eq!(recommended_sleep_for_days(15), 30);
        assert_eq!(recommended_sleep_for_days(16), 60);
        assert_eq!(recommended_sleep_for_days(30), 60);
        assert_eq!(recommended_sleep_for_days(31), 120);
        assert_eq!(recommended_sleep_for_days(90), 120);
        assert_eq!(recommended_sleep_for_days(91), 300);
        assert_eq!(recommended_sleep_for_days(365), 300);
    }

    #[test]
    fn get_offset_from_tz_utc() {
        let offset = get_offset_from_tz("UTC").unwrap();
        assert_eq!(offset, "+00:00");
    }

    #[test]
    fn get_offset_from_tz_rejects_invalid() {
        assert!(get_offset_from_tz("Not/ATimezone").is_err());
    }
}
