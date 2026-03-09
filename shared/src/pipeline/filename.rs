//! Output filename formatting from strftime patterns and custom tokens.

use chrono::{DateTime, Local};

/// Format an output filename from a strftime pattern, a date, a MAC address,
/// and an optional station name.
///
/// Tokens replaced before strftime formatting:
/// - `{mac}` → normalized MAC address (`AA-BB-CC-DD-EE-FF`)
/// - `{station}` → `station_name` if non-empty, otherwise the normalized MAC
///
/// # Examples
///
/// ```
/// use chrono::{Local, TimeZone};
/// use shared::pipeline::filename::format_output_filename;
///
/// let dt = Local.with_ymd_and_hms(2024, 5, 1, 12, 0, 0).unwrap();
/// assert_eq!(
///     format_output_filename("%Y-%m-%d.json", &dt, "", ""),
///     "2024-05-01.json"
/// );
/// ```
pub fn format_output_filename(
    pattern: &str,
    date: &DateTime<Local>,
    mac: &str,
    station_name: &str,
) -> String {
    let normalized = mac
        .chars()
        .filter(char::is_ascii_hexdigit)
        .collect::<String>()
        .to_uppercase();

    // Re-insert dashes every two hex digits: AABBCCDDEEFF → AA-BB-CC-DD-EE-FF
    let mac_dashed = normalized
        .as_bytes()
        .chunks(2)
        .map(|c| std::str::from_utf8(c).unwrap_or("??"))
        .collect::<Vec<_>>()
        .join("-");

    let station = if station_name.is_empty() {
        mac_dashed.as_str()
    } else {
        station_name
    };

    #[allow(clippy::literal_string_with_formatting_args)]
    let with_tokens = pattern
        .replace("{mac}", &mac_dashed)
        .replace("{station}", station);

    date.format(&with_tokens).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn default_pattern() {
        let dt = Local.with_ymd_and_hms(2024, 5, 1, 12, 0, 0).unwrap();
        assert_eq!(
            format_output_filename("%Y-%m-%d.json", &dt, "", ""),
            "2024-05-01.json"
        );
    }

    #[test]
    fn with_mac_colon_separated() {
        let dt = Local.with_ymd_and_hms(2024, 5, 1, 0, 0, 0).unwrap();
        let result = format_output_filename("{mac}-%Y-%m-%d.json", &dt, "AA:BB:CC:DD:EE:FF", "");
        assert_eq!(result, "AA-BB-CC-DD-EE-FF-2024-05-01.json");
    }

    #[test]
    fn with_mac_no_separator() {
        let dt = Local.with_ymd_and_hms(2024, 5, 1, 0, 0, 0).unwrap();
        let result = format_output_filename("{mac}-%Y-%m-%d.json", &dt, "aabbccddeeff", "");
        assert_eq!(result, "AA-BB-CC-DD-EE-FF-2024-05-01.json");
    }

    #[test]
    fn subdir_pattern() {
        let dt = Local.with_ymd_and_hms(2024, 5, 1, 0, 0, 0).unwrap();
        let result = format_output_filename("%Y/%m/%Y-%m-%d.json", &dt, "", "");
        assert_eq!(result, "2024/05/2024-05-01.json");
    }

    #[test]
    fn mac_and_subdir() {
        let dt = Local.with_ymd_and_hms(2024, 5, 1, 0, 0, 0).unwrap();
        let result =
            format_output_filename("%Y/%m/{mac}-%Y-%m-%d.json", &dt, "AA:BB:CC:DD:EE:FF", "");
        assert_eq!(result, "2024/05/AA-BB-CC-DD-EE-FF-2024-05-01.json");
    }

    #[test]
    fn station_used_when_set() {
        let dt = Local.with_ymd_and_hms(2024, 5, 1, 0, 0, 0).unwrap();
        let result =
            format_output_filename("{station}-%Y-%m-%d.json", &dt, "AA:BB:CC:DD:EE:FF", "roof");
        assert_eq!(result, "roof-2024-05-01.json");
    }

    #[test]
    fn station_falls_back_to_mac() {
        let dt = Local.with_ymd_and_hms(2024, 5, 1, 0, 0, 0).unwrap();
        let result =
            format_output_filename("{station}-%Y-%m-%d.json", &dt, "AA:BB:CC:DD:EE:FF", "");
        assert_eq!(result, "AA-BB-CC-DD-EE-FF-2024-05-01.json");
    }

    #[test]
    fn both_tokens() {
        let dt = Local.with_ymd_and_hms(2024, 5, 1, 0, 0, 0).unwrap();
        let result = format_output_filename(
            "{station}/{mac}-%Y-%m-%d.json",
            &dt,
            "AA:BB:CC:DD:EE:FF",
            "roof",
        );
        assert_eq!(result, "roof/AA-BB-CC-DD-EE-FF-2024-05-01.json");
    }
}
