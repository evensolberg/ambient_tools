//! Gets device information.

use std::io::Write;

use crate::creds::Query;
use reqwest::Error;

/// Get the device information.
pub async fn download_device_info(creds: &Query) -> Result<String, Error> {
    let url = format!(
        "https://rt.ambientweather.net/v1/devices?applicationKey={}&apiKey={}",
        creds.app_key, creds.api_key
    );

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    log::debug!("resp = {resp:?}");

    let res = resp.text().await?;
    log::debug!("res = {res:?}",);

    Ok(res)
}

/// Write the information to a JSON file
pub fn write_device_info_to_file(
    filename: &str,
    device_info: &str,
) -> Result<usize, std::io::Error> {
    let mut file = std::fs::File::create(filename)?;
    let res = file.write(device_info.as_bytes())?;

    log::debug!("Wrote {res} bytes to {filename}.");

    Ok(res)
}

/// Get the device information. Return the number of bytes written. Print a summary (i..e, the device information) if requested.
pub async fn get_device_info(
    creds: &Query,
    filename: &str,
    print_summary: bool,
) -> Result<usize, Box<dyn std::error::Error>> {
    let device_info = download_device_info(creds).await?;

    if print_summary {
        println!("{device_info}");
    }

    let bytes_written = write_device_info_to_file(filename, &device_info)?;
    Ok(bytes_written)
}
