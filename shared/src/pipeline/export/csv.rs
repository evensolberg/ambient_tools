//! CSV export for `WeatherDataPoint` records.
//!
//! Default field set covers the common outdoor/indoor readings. Pass `Some(&["all"])` as
//! `fields` to export every field, or supply a list of column names to select specific ones.

use std::io::Write;

use anyhow::Result;

use crate::datapoint::{
    data_point::WeatherDataPoint, length::Length, pressure::AirPressure, speed::WindSpeed,
    temperature::Temperature, units::SystemOfUnits,
};
use crate::pipeline::filter::ms_to_datetime;

/// Default columns exported when no `--fields` override is given.
pub const DEFAULT_FIELDS: &[&str] = &[
    "date",
    "temp_out",
    "humidity_out",
    "wind_speed",
    "wind_gust",
    "wind_dir",
    "baro_rel",
    "solar_radiation",
    "uv",
    "hourly_rain",
    "daily_rain",
    "temp_in",
    "humidity_in",
];

/// All available column names (for `--fields all`).
pub const ALL_FIELDS: &[&str] = &[
    "date",
    "temp_out",
    "humidity_out",
    "wind_speed",
    "wind_speed_2min",
    "wind_speed_10min",
    "wind_gust",
    "wind_gust_daily_max",
    "wind_dir",
    "wind_dir_2min",
    "wind_dir_10min",
    "wind_gust_dir",
    "baro_rel",
    "baro_abs",
    "solar_radiation",
    "uv",
    "hourly_rain",
    "event_rain",
    "daily_rain",
    "weekly_rain",
    "monthly_rain",
    "yearly_rain",
    "total_rain",
    "last_24h_rain",
    "temp_in",
    "humidity_in",
    "co2",
    "last_rain",
    "temp_1",
    "humidity_1",
    "temp_2",
    "humidity_2",
    "temp_3",
    "humidity_3",
    "temp_4",
    "humidity_4",
    "temp_5",
    "humidity_5",
    "temp_6",
    "humidity_6",
    "temp_7",
    "humidity_7",
    "temp_8",
    "humidity_8",
    "temp_9",
    "humidity_9",
    "temp_10",
    "humidity_10",
];

/// Write weather records as CSV.
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
pub fn write_csv<W: Write>(
    writer: W,
    records: &[WeatherDataPoint],
    fields: Option<&[&str]>,
    units: SystemOfUnits,
) -> Result<()> {
    let resolved: &[&str] = match fields {
        None => DEFAULT_FIELDS,
        Some(f) if f == ["all"] => ALL_FIELDS,
        Some(f) => f,
    };

    let mut wtr = csv::Writer::from_writer(writer);

    // Header
    wtr.write_record(resolved)?;

    // Rows
    for rec in records {
        let row: Vec<String> = resolved.iter().map(|col| cell(rec, col, units)).collect();
        wtr.write_record(&row)?;
    }

    wtr.flush()?;
    Ok(())
}

/// Returns the subset of [`ALL_FIELDS`] that have at least one non-empty value across `records`.
///
/// Useful for discovering which fields are actually populated by a particular station before
/// building a `[process.convert] fields = [...]` config entry.
pub fn fields_with_data(records: &[WeatherDataPoint], units: SystemOfUnits) -> Vec<&'static str> {
    ALL_FIELDS
        .iter()
        .copied()
        .filter(|&f| records.iter().any(|r| !cell(r, f, units).is_empty()))
        .collect()
}

/// Render a single cell value for a given column name and record.
#[allow(clippy::too_many_lines)]
pub(super) fn cell(r: &WeatherDataPoint, col: &str, units: SystemOfUnits) -> String {
    match col {
        "date" => r
            .dateutc
            .map(|ms| {
                ms_to_datetime(ms)
                    .format("%Y-%m-%d %H:%M:%S UTC")
                    .to_string()
            })
            .unwrap_or_default(),

        "temp_out" => r
            .temperature
            .map(|t: Temperature| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", t.to_celsius())
                } else {
                    format!("{:.1}", t.to_fahrenheit())
                }
            })
            .unwrap_or_default(),

        "humidity_out" => r.humidity.map(|h: u8| h.to_string()).unwrap_or_default(),

        "wind_speed" => r
            .wind_speed
            .map(|w: WindSpeed| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", w.to_kph())
                } else {
                    format!("{:.1}", w.to_mph())
                }
            })
            .unwrap_or_default(),

        "wind_speed_2min" => r
            .wind_speed_2min_average
            .map(|w: WindSpeed| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", w.to_kph())
                } else {
                    format!("{:.1}", w.to_mph())
                }
            })
            .unwrap_or_default(),

        "wind_speed_10min" => r
            .wind_speed_10min_average
            .map(|w: WindSpeed| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", w.to_kph())
                } else {
                    format!("{:.1}", w.to_mph())
                }
            })
            .unwrap_or_default(),

        "wind_gust" => r
            .wind_gust_speed
            .map(|w: WindSpeed| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", w.to_kph())
                } else {
                    format!("{:.1}", w.to_mph())
                }
            })
            .unwrap_or_default(),

        "wind_gust_daily_max" => r
            .wind_gust_speed_daily_max
            .map(|w: WindSpeed| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", w.to_kph())
                } else {
                    format!("{:.1}", w.to_mph())
                }
            })
            .unwrap_or_default(),

        "wind_dir" => r.wind_direction.map(|d| format!("{d}")).unwrap_or_default(),

        "wind_dir_2min" => r
            .wind_direction_2min_average
            .map(|d| format!("{d}"))
            .unwrap_or_default(),

        "wind_dir_10min" => r
            .wind_direction_10min_average
            .map(|d| format!("{d}"))
            .unwrap_or_default(),

        "wind_gust_dir" => r
            .wind_gust_direction
            .map(|d| format!("{d}"))
            .unwrap_or_default(),

        "baro_rel" => r
            .indoor_relative_pressure
            .map(|p: AirPressure| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", p.to_hpa())
                } else {
                    format!("{:.3}", p.to_inhg())
                }
            })
            .unwrap_or_default(),

        "baro_abs" => r
            .indoor_absolute_pressure
            .map(|p: AirPressure| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", p.to_hpa())
                } else {
                    format!("{:.3}", p.to_inhg())
                }
            })
            .unwrap_or_default(),

        "solar_radiation" => r
            .solar_radiation
            .map(|s: f32| format!("{s:.1}"))
            .unwrap_or_default(),

        "uv" => r.uv_index.map(|u: u8| u.to_string()).unwrap_or_default(),

        "hourly_rain" => r
            .rain_hourly_rate
            .map(|l: Length| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", l.to_millimeters())
                } else {
                    format!("{:.3}", l.to_inches())
                }
            })
            .unwrap_or_default(),

        "event_rain" => r
            .rain_event_amount
            .map(|l: Length| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", l.to_millimeters())
                } else {
                    format!("{:.3}", l.to_inches())
                }
            })
            .unwrap_or_default(),

        "daily_rain" => r
            .rain_daily
            .map(|l: Length| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", l.to_millimeters())
                } else {
                    format!("{:.3}", l.to_inches())
                }
            })
            .unwrap_or_default(),

        "weekly_rain" => r
            .rain_weekly
            .map(|l: Length| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", l.to_millimeters())
                } else {
                    format!("{:.3}", l.to_inches())
                }
            })
            .unwrap_or_default(),

        "monthly_rain" => r
            .rain_monthly
            .map(|l: Length| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", l.to_millimeters())
                } else {
                    format!("{:.3}", l.to_inches())
                }
            })
            .unwrap_or_default(),

        "yearly_rain" => r
            .rain_yearly
            .map(|l: Length| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", l.to_millimeters())
                } else {
                    format!("{:.3}", l.to_inches())
                }
            })
            .unwrap_or_default(),

        "total_rain" => r
            .rain_total
            .map(|l: Length| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", l.to_millimeters())
                } else {
                    format!("{:.3}", l.to_inches())
                }
            })
            .unwrap_or_default(),

        "last_24h_rain" => r
            .last_24_hour_rain
            .map(|l: Length| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", l.to_millimeters())
                } else {
                    format!("{:.3}", l.to_inches())
                }
            })
            .unwrap_or_default(),

        "temp_in" => r
            .temp
            .map(|t: Temperature| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", t.to_celsius())
                } else {
                    format!("{:.1}", t.to_fahrenheit())
                }
            })
            .unwrap_or_default(),

        "humidity_in" => r
            .indoor_humidity_percent
            .map(|h: u8| h.to_string())
            .unwrap_or_default(),

        "co2" => r.co2_ppm.map(|c: u16| c.to_string()).unwrap_or_default(),

        "last_rain" => r
            .last_rain
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_default(),

        "temp_1" => r
            .temp_1
            .map(|t: Temperature| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", t.to_celsius())
                } else {
                    format!("{:.1}", t.to_fahrenheit())
                }
            })
            .unwrap_or_default(),
        "humidity_1" => r.humidity_1.map(|h: u8| h.to_string()).unwrap_or_default(),
        "temp_2" => r
            .temp_2
            .map(|t: Temperature| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", t.to_celsius())
                } else {
                    format!("{:.1}", t.to_fahrenheit())
                }
            })
            .unwrap_or_default(),
        "humidity_2" => r.humidity_2.map(|h: u8| h.to_string()).unwrap_or_default(),
        "temp_3" => r
            .temp_3
            .map(|t: Temperature| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", t.to_celsius())
                } else {
                    format!("{:.1}", t.to_fahrenheit())
                }
            })
            .unwrap_or_default(),
        "humidity_3" => r.humidity_3.map(|h: u8| h.to_string()).unwrap_or_default(),
        "temp_4" => r
            .temp_4
            .map(|t: Temperature| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", t.to_celsius())
                } else {
                    format!("{:.1}", t.to_fahrenheit())
                }
            })
            .unwrap_or_default(),
        "humidity_4" => r.humidity_4.map(|h: u8| h.to_string()).unwrap_or_default(),
        "temp_5" => r
            .temp_5
            .map(|t: Temperature| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", t.to_celsius())
                } else {
                    format!("{:.1}", t.to_fahrenheit())
                }
            })
            .unwrap_or_default(),
        "humidity_5" => r.humidity_5.map(|h: u8| h.to_string()).unwrap_or_default(),
        "temp_6" => r
            .temp_6
            .map(|t: Temperature| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", t.to_celsius())
                } else {
                    format!("{:.1}", t.to_fahrenheit())
                }
            })
            .unwrap_or_default(),
        "humidity_6" => r.humidity_6.map(|h: u8| h.to_string()).unwrap_or_default(),
        "temp_7" => r
            .temp_7
            .map(|t: Temperature| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", t.to_celsius())
                } else {
                    format!("{:.1}", t.to_fahrenheit())
                }
            })
            .unwrap_or_default(),
        "humidity_7" => r.humidity_7.map(|h: u8| h.to_string()).unwrap_or_default(),
        "temp_8" => r
            .temp_8
            .map(|t: Temperature| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", t.to_celsius())
                } else {
                    format!("{:.1}", t.to_fahrenheit())
                }
            })
            .unwrap_or_default(),
        "humidity_8" => r.humidity_8.map(|h: u8| h.to_string()).unwrap_or_default(),
        "temp_9" => r
            .temp_9
            .map(|t: Temperature| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", t.to_celsius())
                } else {
                    format!("{:.1}", t.to_fahrenheit())
                }
            })
            .unwrap_or_default(),
        "humidity_9" => r.humidity_9.map(|h: u8| h.to_string()).unwrap_or_default(),
        "temp_10" => r
            .temp_10
            .map(|t: Temperature| {
                if units == SystemOfUnits::SI {
                    format!("{:.1}", t.to_celsius())
                } else {
                    format!("{:.1}", t.to_fahrenheit())
                }
            })
            .unwrap_or_default(),
        "humidity_10" => r.humidity_10.map(|h: u8| h.to_string()).unwrap_or_default(),

        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datapoint::{
        length::Length, pressure::AirPressure, speed::WindSpeed, temperature::Temperature,
    };

    fn record_with_basics() -> WeatherDataPoint {
        WeatherDataPoint {
            dateutc: Some(1_714_521_600_000), // 2024-05-01 00:00:00 UTC
            temperature: Some(Temperature::Fahrenheit(72.5)),
            humidity: Some(55),
            wind_speed: Some(WindSpeed::MPH(10.0)),
            wind_gust_speed: Some(WindSpeed::MPH(15.0)),
            rain_daily: Some(Length::Inches(0.25)),
            indoor_relative_pressure: Some(AirPressure::inHg(29.92)),
            solar_radiation: Some(350.0),
            uv_index: Some(3),
            temp: Some(Temperature::Fahrenheit(68.0)),
            indoor_humidity_percent: Some(45),
            ..Default::default()
        }
    }

    #[test]
    fn default_fields_header_and_row() {
        let records = vec![record_with_basics()];
        let mut buf = Vec::new();
        write_csv(&mut buf, &records, None, SystemOfUnits::Imperial).unwrap();
        let csv = String::from_utf8(buf).unwrap();
        assert!(csv.contains("date,temp_out,humidity_out"));
        assert!(csv.contains("2024-05-01 00:00:00 UTC"));
        assert!(csv.contains("72.5"));
        assert!(csv.contains("55"));
    }

    #[test]
    fn si_units_converts_temperature() {
        let records = vec![record_with_basics()];
        let mut buf = Vec::new();
        write_csv(&mut buf, &records, None, SystemOfUnits::SI).unwrap();
        let csv = String::from_utf8(buf).unwrap();
        // 72.5°F ≈ 22.5°C
        assert!(csv.contains("22.5"));
    }

    #[test]
    fn si_units_converts_wind_speed() {
        let records = vec![record_with_basics()];
        let mut buf = Vec::new();
        write_csv(&mut buf, &records, Some(&["wind_speed"]), SystemOfUnits::SI).unwrap();
        let csv = String::from_utf8(buf).unwrap();
        // 10 mph ≈ 16.1 kph
        assert!(csv.contains("16.1"));
    }

    #[test]
    fn all_fields_includes_extra_columns() {
        let records = vec![record_with_basics()];
        let mut buf = Vec::new();
        write_csv(&mut buf, &records, Some(&["all"]), SystemOfUnits::Imperial).unwrap();
        let csv = String::from_utf8(buf).unwrap();
        assert!(csv.contains("weekly_rain"));
        assert!(csv.contains("yearly_rain"));
    }

    #[test]
    fn missing_field_produces_empty_cell() {
        let rec = WeatherDataPoint {
            dateutc: Some(1_714_521_600_000),
            ..Default::default()
        };
        let mut buf = Vec::new();
        write_csv(
            &mut buf,
            &[rec],
            Some(&["temp_out", "humidity_out"]),
            SystemOfUnits::Imperial,
        )
        .unwrap();
        let csv = String::from_utf8(buf).unwrap();
        // Both fields are None → two empty cells per row
        let row = csv.lines().nth(1).unwrap();
        assert_eq!(row, ",");
    }

    #[test]
    fn unknown_field_produces_empty_cell() {
        let records = vec![record_with_basics()];
        let mut buf = Vec::new();
        write_csv(
            &mut buf,
            &records,
            Some(&["nonexistent"]),
            SystemOfUnits::Imperial,
        )
        .unwrap();
        let csv = String::from_utf8(buf).unwrap();
        let row = csv.lines().nth(1).unwrap();
        // The csv crate may quote empty strings as "" — either is acceptable
        assert!(row.is_empty() || row == "\"\"");
    }
}
