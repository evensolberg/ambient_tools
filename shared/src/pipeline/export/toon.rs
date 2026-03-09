//! TOON (Token-Oriented Object Notation) export for `WeatherDataPoint` records.
//!
//! TOON encodes a uniform array of objects as a compact tabular block:
//!
//! ```text
//! {date,temp_in,humidity_in}
//! 2026-02-01 08:00:00 UTC,65.5,50
//! 2026-02-01 08:05:00 UTC,65.5,50
//! ```
//!
//! This is 30–60% more token-efficient than JSON and more compact than CSV for
//! uniform data, making it well-suited for feeding weather records into LLMs.

use std::io::Write;

use anyhow::Result;

use crate::datapoint::{data_point::WeatherDataPoint, units::SystemOfUnits};

use super::csv::{cell, ALL_FIELDS, DEFAULT_FIELDS};

/// Write weather records in TOON format.
///
/// # Arguments
///
/// * `writer` — destination (file or stdout)
/// * `records` — records to export
/// * `fields` — column list; `None` uses [`DEFAULT_FIELDS`]; `Some(&["all"])` exports [`ALL_FIELDS`]
/// * `units` — `SI` converts temperatures to °C, pressures to hPa, speeds to kph, lengths to mm
///
/// # Errors
///
/// Returns an error if writing to `writer` fails.
pub fn write_toon<W: Write>(
    mut writer: W,
    records: &[WeatherDataPoint],
    fields: Option<&[&str]>,
    units: SystemOfUnits,
) -> Result<()> {
    let resolved: &[&str] = match fields {
        None => DEFAULT_FIELDS,
        Some(f) if f == ["all"] => ALL_FIELDS,
        Some(f) => f,
    };

    // TOON header: {field1,field2,...}
    writeln!(writer, "{{{}}}", resolved.join(","))?;

    // Data rows — comma-separated values, no quoting needed for weather data
    for rec in records {
        let row: Vec<String> = resolved.iter().map(|col| cell(rec, col, units)).collect();
        writeln!(writer, "{}", row.join(","))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datapoint::{
        data_point::WeatherDataPoint, length::Length, pressure::AirPressure, speed::WindSpeed,
        temperature::Temperature, units::SystemOfUnits,
    };

    fn record_with_basics() -> WeatherDataPoint {
        WeatherDataPoint {
            dateutc: Some(1_714_521_600_000), // 2024-05-01 00:00:00 UTC
            temperature: Some(Temperature::Fahrenheit(72.5)),
            humidity: Some(55),
            wind_speed: Some(WindSpeed::MPH(10.0)),
            indoor_relative_pressure: Some(AirPressure::inHg(29.92)),
            rain_daily: Some(Length::Inches(0.25)),
            temp: Some(Temperature::Fahrenheit(68.0)),
            indoor_humidity_percent: Some(45),
            ..Default::default()
        }
    }

    #[test]
    fn toon_header_uses_braces() {
        let mut buf = Vec::new();
        write_toon(
            &mut buf,
            &[record_with_basics()],
            Some(&["date", "temp_out"]),
            SystemOfUnits::Imperial,
        )
        .unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.starts_with("{date,temp_out}"));
    }

    #[test]
    fn toon_row_contains_values() {
        let mut buf = Vec::new();
        write_toon(
            &mut buf,
            &[record_with_basics()],
            Some(&["date", "temp_out"]),
            SystemOfUnits::Imperial,
        )
        .unwrap();
        let output = String::from_utf8(buf).unwrap();
        let row = output.lines().nth(1).unwrap();
        assert!(row.contains("2024-05-01 00:00:00 UTC"));
        assert!(row.contains("72.5"));
    }

    #[test]
    fn toon_si_converts_temperature() {
        let mut buf = Vec::new();
        write_toon(
            &mut buf,
            &[record_with_basics()],
            Some(&["temp_out"]),
            SystemOfUnits::SI,
        )
        .unwrap();
        let output = String::from_utf8(buf).unwrap();
        // 72.5°F ≈ 22.5°C
        assert!(output.contains("22.5"));
    }

    #[test]
    fn toon_default_fields_header() {
        let mut buf = Vec::new();
        write_toon(
            &mut buf,
            &[record_with_basics()],
            None,
            SystemOfUnits::Imperial,
        )
        .unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.starts_with("{date,temp_out,humidity_out"));
    }
}
