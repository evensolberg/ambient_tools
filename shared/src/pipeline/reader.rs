//! Read Ambient Weather JSON files into `WeatherDataPoint` records.

use std::{
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::datapoint::data_point::WeatherDataPoint;

/// Read a single JSON file into a `Vec<WeatherDataPoint>`.
///
/// Expects the file to contain a JSON array of weather records.
/// Records that fail to deserialize are skipped with a warning.
/// Returns an error if the file cannot be opened or if the outer array is malformed.
pub fn read_file(path: &Path) -> Result<Vec<WeatherDataPoint>> {
    let file = std::fs::File::open(path)
        .with_context(|| format!("Failed to open {}", path.display()))?;
    let reader = BufReader::new(file);
    let raw: Vec<serde_json::Value> = serde_json::from_reader(reader)
        .with_context(|| format!("Failed to parse JSON array in {}", path.display()))?;
    let records = raw
        .into_iter()
        .filter_map(|v| match serde_json::from_value::<WeatherDataPoint>(v) {
            Ok(r) => Some(r),
            Err(e) => {
                log::warn!("Skipping malformed record in {}: {e}", path.display());
                None
            }
        })
        .collect();
    Ok(records)
}

/// Expand a glob pattern and read all matching JSON files.
///
/// Files that fail to parse are skipped with a warning rather than
/// aborting the entire run. Records from all files are merged into a
/// single `Vec`, sorted by `dateutc` (ascending).
pub fn read_glob(pattern: &str) -> Result<Vec<WeatherDataPoint>> {
    let paths = expand_glob(pattern)?;
    if paths.is_empty() {
        anyhow::bail!("No files matched pattern: {pattern}");
    }
    let mut all: Vec<WeatherDataPoint> = Vec::new();
    for path in &paths {
        match read_file(path) {
            Ok(mut records) => all.append(&mut records),
            Err(e) => log::warn!("Skipping {}: {e}", path.display()),
        }
    }
    all.sort_by_key(|r| r.dateutc);
    Ok(all)
}

/// Expand a glob pattern into a sorted list of paths.
fn expand_glob(pattern: &str) -> Result<Vec<PathBuf>> {
    let entries = glob::glob(pattern)
        .with_context(|| format!("Invalid glob pattern: {pattern}"))?;
    let mut paths: Vec<PathBuf> = entries
        .filter_map(|e| match e {
            Ok(p) => Some(p),
            Err(e) => {
                log::warn!("Glob error: {e}");
                None
            }
        })
        .collect();
    paths.sort();
    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp_json(dir: &tempfile::TempDir, name: &str, content: &str) -> PathBuf {
        let path = dir.path().join(name);
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(content.as_bytes()).unwrap();
        path
    }

    #[test]
    fn read_file_empty_array() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_temp_json(&dir, "empty.json", "[]");
        let records = read_file(&path).unwrap();
        assert!(records.is_empty());
    }

    #[test]
    fn read_file_single_record() {
        let dir = tempfile::tempdir().unwrap();
        let json = r#"[{"dateutc": 1700000000000}]"#;
        let path = write_temp_json(&dir, "single.json", json);
        let records = read_file(&path).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].dateutc, Some(1_700_000_000_000));
    }

    #[test]
    fn read_file_missing_returns_error() {
        let result = read_file(Path::new("/nonexistent/path.json"));
        assert!(result.is_err());
    }

    #[test]
    fn read_file_bad_json_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_temp_json(&dir, "bad.json", "not json");
        let result = read_file(&path);
        assert!(result.is_err());
    }

    #[test]
    fn read_glob_skips_bad_file_and_returns_good() {
        let dir = tempfile::tempdir().unwrap();
        write_temp_json(&dir, "good.json", r#"[{"dateutc": 1700000000000}]"#);
        write_temp_json(&dir, "bad.json", "not json");
        let pattern = format!("{}/*.json", dir.path().display());
        let records = read_glob(&pattern).unwrap();
        assert_eq!(records.len(), 1);
    }

    #[test]
    fn read_glob_no_match_returns_error() {
        let result = read_glob("/nonexistent/path/*.json");
        assert!(result.is_err());
    }

    #[test]
    fn read_glob_sorts_by_dateutc() {
        let dir = tempfile::tempdir().unwrap();
        write_temp_json(&dir, "a.json", r#"[{"dateutc": 1700000002000}]"#);
        write_temp_json(&dir, "b.json", r#"[{"dateutc": 1700000001000}]"#);
        let pattern = format!("{}/*.json", dir.path().display());
        let records = read_glob(&pattern).unwrap();
        assert_eq!(records.len(), 2);
        assert!(records[0].dateutc < records[1].dateutc);
    }
}
