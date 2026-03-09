---
id: at-c7p
title: Build data processing pipeline on top of datapoint types
status: closed
type: feature
priority: 4
tags:
- shared
- datapoint
- json
- reporting
created: 2026-03-08
updated: 2026-03-08
closed_reason: ''
dependencies: []
---

# Build data processing pipeline on top of datapoint types

## Implementation Plan

### Relationship to shared::datapoint

The pipeline is built entirely on top of `shared::datapoint`. The existing types are not duplicated or replaced — they are the core of the pipeline:

- `WeatherDataPoint` (`data_point.rs`) is the central type: JSON deserializes directly into it
- `Temperature`, `AirPressure`, `WindSpeed`, `Length` provide `.to(SystemOfUnits)` conversions used during CSV export
- `WindDirection::from_degrees()` converts raw degree readings to compass labels
- `SystemOfUnits` controls the conversion target (SI vs Imperial)

The new `shared::pipeline` module is a processing layer that *consumes* these types — it adds file I/O, date filtering, and CSV export on top of the already-complete type system.

### Architecture

New binary crate `ambient_process` alongside `ambient_download`. Core processing logic lives in a new `shared::pipeline` module.

```
ambient_tools/
  ambient_process/        <- new binary crate
    src/
      main.rs
      cli.rs
  shared/src/
    datapoint/            <- EXISTING (unchanged)
    pipeline/             <- NEW processing layer
      mod.rs
      reader.rs           <- JSON files -> Vec<WeatherDataPoint>
      filter.rs           <- date range filtering
      export/
        mod.rs
        csv.rs            <- CSV writer using datapoint conversion enums
```

### CLI: convert subcommand (initial scope)

```
ambient_process convert [OPTIONS] <FILES|GLOB>

  -o, --output <FILE>        Output CSV [default: stdout]
  --from <DATE>              Filter on or after YYYY-MM-DD
  --to <DATE>                Filter on or before YYYY-MM-DD
  --units <si|imperial>      Unit system [default: imperial]
  --fields <f1,f2,...>       Override default field list; 'all' exports everything
  -c, --config-file <FILE>   Optional TOML config
```

A `summary` subcommand (per-day/per-month stats) is deferred to a follow-up.

### shared::pipeline modules

**reader.rs**
```rust
pub fn read_file(path: &Path) -> Result<Vec<WeatherDataPoint>>
pub fn read_dir(dir: &Path, pattern: &str) -> Result<Vec<WeatherDataPoint>>
```
Sort by dateutc after reading. Skip and warn on malformed files.

**filter.rs**
```rust
pub struct DateFilter { pub from: Option<NaiveDate>, pub to: Option<NaiveDate> }
impl DateFilter { pub fn apply(&self, records: Vec<WeatherDataPoint>) -> Vec<WeatherDataPoint> }
```
dateutc is milliseconds UTC epoch. Records without a parseable date pass through with a warning.

**export/csv.rs** — use the `csv` crate
```rust
pub fn write_csv<W: Write>(writer: W, records: &[WeatherDataPoint], fields: Option<&[&str]>, units: SystemOfUnits) -> Result<()>
```

Default field set (curated): date, temp_out_f, humidity_out, feels_like_out, dew_point_out, wind_speed_mph, wind_gust_mph, wind_dir, baro_rel_in, solar_radiation, uv, hourly_rain_in, daily_rain_in, temp_in_f, humidity_in

Unit conversions: Temperature -> Celsius (SI), AirPressure -> hPa (SI), WindSpeed -> KPH (SI), Length -> mm (SI).

### New workspace dependencies
- `csv` — CSV serialization
- `glob` — file pattern expansion

### Pre-work: private fields
Several WeatherDataPoint fields are currently private (e.g. wind_speed). Audit data_point.rs and make all fields pub before implementing pipeline.

### Testing
- read_file against a JSON fixture
- DateFilter::apply with synthetic records
- write_csv against expected headers/values
- Integration: round-trip a real Weather directory JSON file

[2026-03-08] Design decision: separate binary (ambient_process) rather than extending ambient_download. JSON files serve as the intermediate format — download once, re-process many times with different options (units, fields, date range) without hitting the API. The two tools compose naturally in shell scripts and cron jobs.

[start] 2026-03-08 21:59:14

[stop]  2026-03-08 22:11:01  11m 47s  Pipeline complete: reader, filter, CSV export, prettify. All 55 tests pass.
