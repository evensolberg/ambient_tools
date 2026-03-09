# ambient_process

Reads Ambient Weather JSON data files (produced by `ambient_download`) and exports them to CSV or pretty-prints them in place. Designed for post-download analysis, scripting, and data exploration.

## Table of Contents

- [ambient\_process](#ambient_process)
  - [Table of Contents](#table-of-contents)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Global options](#global-options)
    - [`convert` subcommand](#convert-subcommand)
      - [Default fields](#default-fields)
      - [Unit conversion](#unit-conversion)
      - [Field selection](#field-selection)
    - [`prettify` subcommand](#prettify-subcommand)

---

## Prerequisites

JSON data files produced by [`ambient_download`](../ambient_download/README.md).

---

## Installation

Build from source using Cargo:

```shell
git clone https://github.com/evensolberg/ambient_tools.git
cd ambient_tools
cargo build --release
```

The compiled binary is placed at `target/release/ambient_process`.

---

## Usage

```text
ambient_process [OPTIONS] [COMMAND]

Options:
  -d, --detail-level <detail>    Output detail level: 0=Quiet 1=Normal 2=Detailed 3=Debug
                                 [default: 1] [env: AMBIENT_WEATHER_DETAIL_LEVEL]

Commands:
  convert   Convert JSON weather files to CSV
  prettify  Pretty-print JSON weather files in place
  help      Print this message or the help of a subcommand
```

### Global options

| Flag | Short | Description |
| --- | --- | --- |
| `--detail-level` | `-d` | Verbosity: 0 silent, 1 normal, 2 detailed, 3 debug |

---

### `convert` subcommand

Reads one or more JSON data files (glob patterns accepted), optionally filters by date range, and writes a combined CSV.

```text
ambient_process convert [OPTIONS] <FILES>...

Arguments:
  <FILES>    Input JSON files or glob patterns (e.g. 'data/*.json')

Options:
  -o, --output <FILE>        Output CSV file. Writes to stdout if omitted.
      --from <DATE>          Include records on or after this date (YYYY-MM-DD)
      --to <DATE>            Include records on or before this date (YYYY-MM-DD)
  -u, --units <si|imperial>  Unit system for exported values [default: imperial]
  -f, --fields <FIELDS>      Comma-separated field list, or 'all' for every field
```

Example — convert all of February to a CSV in SI units:

```shell
ambient_process convert 'data/2026-02-*.json' \
  --output february.csv \
  --units si
```

Example — export a single day to stdout with custom fields:

```shell
ambient_process convert data/2026-02-14.json \
  --fields date,temp_in,humidity_in,baro_rel,daily_rain
```

Example — extract a date range from a full-year glob:

```shell
ambient_process convert 'data/2026-*.json' \
  --from 2026-01-01 --to 2026-03-31 \
  --output q1.csv
```

#### Default fields

When `--fields` is not supplied, the following columns are exported:

| Column | Description |
| --- | --- |
| `date` | UTC timestamp (YYYY-MM-DD HH:MM:SS UTC) |
| `temp_out` | Outdoor temperature |
| `humidity_out` | Outdoor humidity (%) |
| `wind_speed` | Wind speed |
| `wind_gust` | Wind gust speed |
| `wind_dir` | Wind direction (compass label) |
| `baro_rel` | Relative barometric pressure |
| `solar_radiation` | Solar radiation (W/m²) |
| `uv` | UV index |
| `hourly_rain` | Hourly rain rate |
| `daily_rain` | Daily rain total |
| `temp_in` | Indoor temperature |
| `humidity_in` | Indoor humidity (%) |

Fields with no data for your station produce empty cells — they are not an error.

#### Unit conversion

| Measurement | Imperial | SI (`--units si`) |
| --- | --- | --- |
| Temperature | °F | °C |
| Wind speed | mph | kph |
| Pressure | inHg | hPa |
| Rain / length | inches | millimeters |

#### Field selection

Use `--fields` with a comma-separated list to choose specific columns:

```shell
--fields date,temp_in,humidity_in,daily_rain
```

Use `--fields all` to export every available field:

```shell
--fields all
```

Available fields beyond the defaults: `wind_speed_2min`, `wind_speed_10min`, `wind_gust_daily_max`, `wind_dir_2min`, `wind_dir_10min`, `wind_gust_dir`, `baro_abs`, `event_rain`, `weekly_rain`, `monthly_rain`, `yearly_rain`, `total_rain`, `last_24h_rain`, `co2`.

---

### `prettify` subcommand

Re-writes JSON files with indented formatting in place. Useful for inspecting raw data files in a text editor.

```text
ambient_process prettify <FILES>...

Arguments:
  <FILES>    Input JSON files or glob patterns
```

Example — prettify all files for a month:

```shell
ambient_process prettify 'data/2026-02-*.json'
```

The original files are overwritten. The JSON content is unchanged — only whitespace formatting is affected.
