//! Gets device information.

use anyhow::{Context, Result};
use std::io::Write;

use shared::config;

use crate::{creds, creds::Query, detail::DetailLevel};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_mac_addresses_single_device() {
        let json = serde_json::json!([{"macAddress": "AA:BB:CC:DD:EE:FF", "info": {}}]);
        assert_eq!(extract_mac_addresses(&json), vec!["AA:BB:CC:DD:EE:FF"]);
    }

    #[test]
    fn extract_mac_addresses_multiple_devices() {
        let json = serde_json::json!([
            {"macAddress": "AA:BB:CC:DD:EE:FF"},
            {"macAddress": "11:22:33:44:55:66"},
        ]);
        assert_eq!(
            extract_mac_addresses(&json),
            vec!["AA:BB:CC:DD:EE:FF", "11:22:33:44:55:66"]
        );
    }

    #[test]
    fn extract_mac_addresses_empty_array() {
        let json = serde_json::json!([]);
        assert!(extract_mac_addresses(&json).is_empty());
    }

    #[test]
    fn extract_mac_addresses_missing_field() {
        let json = serde_json::json!([{"info": {}}]);
        assert!(extract_mac_addresses(&json).is_empty());
    }

    #[test]
    fn extract_mac_addresses_not_an_array() {
        let json = serde_json::json!({"macAddress": "AA:BB:CC:DD:EE:FF"});
        assert!(extract_mac_addresses(&json).is_empty());
    }

    #[test]
    fn additional_macs_appended_as_comments() {
        let dir = tempfile::tempdir().unwrap();
        let config_path = dir.path().join("test.toml");
        std::fs::write(&config_path, "mac_address = \"AA:BB:CC:DD:EE:FF\"\n").unwrap();

        let extra = ["11:22:33:44:55:66", "77:88:99:AA:BB:CC"];
        let mut f = std::fs::OpenOptions::new()
            .append(true)
            .open(&config_path)
            .unwrap();
        writeln!(f, "\n# Additional MAC addresses found:").unwrap();
        for mac in &extra {
            writeln!(f, "# mac_address = \"{mac}\"").unwrap();
        }
        drop(f);

        let contents = std::fs::read_to_string(&config_path).unwrap();
        assert!(contents.contains("# mac_address = \"11:22:33:44:55:66\""));
        assert!(contents.contains("# mac_address = \"77:88:99:AA:BB:CC\""));
    }
}

/// Extract MAC addresses from a parsed Ambient Weather device list response.
fn extract_mac_addresses(value: &serde_json::Value) -> Vec<String> {
    value
        .as_array()
        .map(|devices| {
            devices
                .iter()
                .filter_map(|d| d["macAddress"].as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

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

    let (_bytes, macs) = download_device_info_to_file(
        &config.output_folder,
        dev_info_file_name,
        detail_level,
        creds,
    )?;

    let save_mac = device_args.get_flag("save-mac");
    if save_mac {
        match macs.as_slice() {
            [] => {
                log::warn!("--save-mac requested but no MAC address found in response.");
            }
            [mac] => {
                let Some(config_file) = cli_args.get_one::<String>("config-file") else {
                    log::warn!(
                        "--save-mac requires a config file (--config-file). \
                         MAC address found: {mac}"
                    );
                    return Ok(());
                };
                let mut updated = config.clone();
                updated.mac_address = mac.clone();
                updated.to_file(config_file)?;
                log::info!("Saved MAC address {mac} to {config_file}.");
            }
            multiple => {
                let first = &multiple[0];
                log::warn!(
                    "Multiple devices found ({}). Saving the first MAC address: {first}",
                    multiple.len()
                );
                let Some(config_file) = cli_args.get_one::<String>("config-file") else {
                    log::warn!(
                        "--save-mac requires a config file (--config-file). \
                         MAC address found: {first}"
                    );
                    return Ok(());
                };
                let mut updated = config.clone();
                updated.mac_address = first.clone();
                updated.to_file(config_file)?;
                log::info!("Saved MAC address {first} to {config_file}.");

                // Append remaining MACs as comments so the user can find them.
                let mut f = std::fs::OpenOptions::new()
                    .append(true)
                    .open(config_file)
                    .with_context(|| format!("Failed to open {config_file} for appending"))?;
                writeln!(f, "\n# Additional MAC addresses found:")?;
                for mac in &multiple[1..] {
                    writeln!(f, "# mac_address = \"{mac}\"")?;
                }
                log::info!(
                    "Appended {} additional MAC address(es) as comments to {config_file}.",
                    multiple.len() - 1
                );
            }
        }
    }

    Ok(())
}

/// Download the device information and write it to a file.
///
/// # Returns
///
/// A `Result` with `(bytes_written, mac_addresses)`.
///
/// # Errors
///
/// If the output folder cannot be created or the device information cannot be downloaded.
fn download_device_info_to_file(
    output_folder: &str,
    dev_info_file_name: &str,
    detail_level: DetailLevel,
    creds: &Query,
) -> Result<(usize, Vec<String>)> {
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
    let value: serde_json::Value = serde_json::from_str(&device_info)
        .with_context(|| format!("Failed to parse device info JSON for {full_path}"))?;

    let macs = extract_mac_addresses(&value);

    let pretty = serde_json::to_string_pretty(&value)
        .with_context(|| format!("Failed to serialize device info JSON for {full_path}"))?;

    let mut file = std::fs::File::create(&full_path)?;
    let bytes_written = file.write(pretty.as_bytes())?;

    if detail_level > DetailLevel::Quiet {
        log::info!("Wrote {bytes_written} bytes to {full_path}.");
    } else {
        log::debug!("Wrote {bytes_written} bytes to {full_path}.");
    }

    Ok((bytes_written, macs))
}
