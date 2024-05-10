//! Gets device information.

use std::io::Write;

use crate::{creds::Query, detail::DetailLevel};
use std::error::Error;

/// Get the device information.
pub async fn download_device_info(creds: &Query) -> Result<String, Box<dyn Error>> {
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
    log::debug!("res = {res:?}");

    Ok(res)
}

/// Get the device information. Return the number of bytes written. Print a summary (i..e, the device information) if requested.
pub async fn get_device_info(
    creds: &Query,
    filename: &str,
    detail_level: DetailLevel,
) -> Result<usize, Box<dyn std::error::Error>> {
    let device_info = download_device_info(creds).await?;

    if detail_level > DetailLevel::Normal {
        log::info!("{device_info}");
    }

    let mut file = std::fs::File::create(filename)?;
    let bytes_written = file.write(device_info.as_bytes())?;
    Ok(bytes_written)
}
