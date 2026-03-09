//! Gets device information.

use anyhow::{Context, Result};
use std::io::Write;

use shared::config;

use crate::{creds, creds::Query, detail::DetailLevel};

/// Get the device information and write it to a file.
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
/// If the device information cannot be downloaded or written to a file.
/// If the output folder cannot be created.
///
/// # Panics
///
/// If the device subcommand is not found.
pub fn get_device_info(
    cli_args: &clap::ArgMatches,
    config: &config::Config,
    creds: &creds::Query,
    detail_level: DetailLevel,
) -> Result<()> {
    if detail_level > DetailLevel::Quiet {
        log::info!("Getting device information.");
    }

    let device_args = cli_args
        .subcommand_matches("device")
        .expect("`device` subcommand not found. Yikes!");

    let info_file_default = String::from("device-info.json");
    let dev_info_file_name = device_args
        .get_one::<String>("device-info-filename")
        .unwrap_or(&info_file_default);

    download_device_info_to_file(
        &config.output_folder,
        dev_info_file_name,
        detail_level,
        creds,
    )?;
    Ok(())
}

/// Download the device information and write it to a file.
///
/// # Arguments
///
/// * `output_folder` - The folder to write the output file to.
/// * `dev_info_file_name` - The name of the file to write the device information to.
/// * `detail_level` - The level of detail to log.
/// * `creds` - The credentials to use.
///
/// # Returns
///
/// A `Result` with the number of bytes written to the file.
///
/// # Errors
///
/// If the output folder cannot be created or the device information cannot be downloaded.
fn download_device_info_to_file(
    output_folder: &str,
    dev_info_file_name: &str,
    detail_level: DetailLevel,
    creds: &Query,
) -> Result<usize> {
    crate::check_or_create_output_folder(output_folder)?;

    let url = format!(
        "https://rt.ambientweather.net/v1/devices?applicationKey={}&apiKey={}",
        creds.app_key, creds.api_key
    );

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("ambient-downloader")
        .build()
        .context("Failed to build HTTP client")?;
    let device_info = client
        .get(url)
        .send()
        .context("Failed to connect to Ambient Weather API")?
        .text()
        .context("Failed to read device info response")?;

    if detail_level > DetailLevel::Normal {
        log::info!("{device_info}");
    } else {
        log::debug!("device_info = {device_info}");
    }

    let full_path = format!("{output_folder}/{dev_info_file_name}");
    let mut file = std::fs::File::create(&full_path)?;
    let bytes_written = file.write(device_info.as_bytes())?;

    if detail_level > DetailLevel::Quiet {
        log::info!("Wrote {bytes_written} bytes to {full_path}.");
    } else {
        log::debug!("Wrote {bytes_written} bytes to {full_path}.");
    }

    Ok(bytes_written)
}
