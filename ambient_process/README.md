# ambient_process

Reads Ambient Weather JSON data files (produced by `ambient_download`) and exports them to CSV or pretty-prints them in place. Designed for post-download analysis, scripting, and data exploration.

## Table of Contents

- [ambient\_process](#ambient_process)
  - [Table of Contents](#table-of-contents)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Global options](#global-options)
    - [Config file](#config-file)
    - [`convert` subcommand](#convert-subcommand)
      - [Default fields](#default-fields)
      - [Unit conversion](#unit-conversion)
      - [Field selection](#field-selection)
    - [`fields` subcommand](#fields-subcommand)
    - [`prettify` subcommand](#prettify-subcommand)
    - [`reorganize` subcommand](#reorganize-subcommand)
      - [Pattern tokens](#pattern-tokens)
      - [Reorganize config file](#reorganize-config-file)
      - [Examples](#examples)

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
  -c, --config <FILE>            TOML config file. Falls back to AMBIENT_WEATHER_CONFIG
                                 env var, then ambient_tools.toml in cwd.

Commands:
  convert     Convert JSON weather files to CSV or TOON
  fields      List fields that have data in one or more JSON files
  prettify    Pretty-print JSON weather files in place
  reorganize  Rename and reorganize JSON weather files using a filename pattern
  help        Print this message or the help of a subcommand
```

### Global options

| Flag | Short | Description |
| --- | --- | --- |
| `--detail-level` | `-d` | Verbosity: 0 silent, 1 normal, 2 detailed, 3 debug |
| `--config` | `-c` | TOML config file (see [Config file](#config-file)) |

---

### Config file

Persistent defaults for `convert` can be stored in a TOML file under a `[process.convert]` section. This avoids repeating flags on every invocation.

Config file resolution order (first match wins):

1. `--config` flag on the subcommand (e.g. `convert -c myfile.toml`)
2. `--config` flag on the top-level command (e.g. `ambient_process -c myfile.toml convert`)
3. `AMBIENT_WEATHER_CONFIG` environment variable
4. `ambient_tools.toml` in the current working directory

CLI flags always override config file values when both are present.

Example `[process.convert]` section:

```toml
[process.convert]
fields = [
  "date",
  "temp_out",
  "humidity_out",
  "wind_speed",
  "daily_rain",
]
format = "csv"
units  = "imperial"
# output = "weather.csv"   # uncomment to default to a file
# from   = "2026-01-01"
# to     = "2026-12-31"
```

Use the [`fields` subcommand](#fields-subcommand) to discover which fields are actually populated by your station before building this list.

The `[process.convert]` section can live in the same file as `[download]` settings for `ambient_download`, giving you a single config file for both tools:

```toml
[download]
app_key          = ""
api_key          = ""
mac_address      = "AA:BB:CC:DD:EE:FF"
output_folder    = "/path/to/weather"
filename_pattern = "%Y/%Y-%m-%d.json"
station_name     = "home"
tz_name          = "America/Vancouver"
detail_level     = 1
limit            = 288
sleep_time       = 30

[process.convert]
fields = [
  "date",
  "temp_out",
  "humidity_out",
  "wind_speed",
  "daily_rain",
]
format = "csv"
units  = "imperial"
# output = "weather.csv"
```

---

### `convert` subcommand

Reads one or more JSON data files (glob patterns accepted), optionally filters by date range, and writes a combined CSV or TOON file.

```text
ambient_process convert [OPTIONS] <FILES>...

Arguments:
  <FILES>    Input JSON files or glob patterns (e.g. 'data/*.json')

Options:
  -o, --output <FILE>          Output file. Writes to stdout if omitted.
      --from <DATE>            Include records on or after this date (YYYY-MM-DD)
      --to <DATE>              Include records on or before this date (YYYY-MM-DD)
  -u, --units <si|imperial>    Unit system for exported values [default: imperial]
  -f, --fields <FIELDS>        Comma-separated field list, or 'all' for every field
      --format <csv|toon>      Output format [default: csv]
  -c, --config <FILE>          TOML config file. Overrides top-level --config.
```

All options except `--output` and `--format` can also be set in the config file (see [Config file](#config-file)). CLI flags take precedence.

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

#### TOON format

[TOON (Token-Oriented Object Notation)](https://github.com/evensolberg/json2toon) is a compact tabular format well-suited for feeding weather records into LLMs. A uniform array of objects is encoded as a header line followed by data rows:

```text
{date,temp_in,humidity_in,baro_rel,daily_rain}
2026-02-01 08:00:00 UTC,65.5,50,29.569,0.000
2026-02-01 08:05:00 UTC,65.5,50,29.575,0.000
```

This is 30–60% more token-efficient than the equivalent JSON. Use `--format toon` to enable it:

```shell
ambient_process convert 'data/2026-02-*.json' \
  --fields date,temp_in,humidity_in,baro_rel,daily_rain \
  --format toon \
  --output february.toon
```

---

### `fields` subcommand

Lists the field names that contain at least one non-empty value across the given JSON files. Output is in TOML array syntax, ready to paste into a `[process.convert]` config section.

```text
ambient_process fields <FILES>...

Arguments:
  <FILES>    Input JSON files or glob patterns
```

Example — discover what your station actually records:

```shell
ambient_process -d 0 fields 'data/*.json'
```

```toml
fields = [
  "date",
  "temp_out",
  "humidity_out",
  "wind_speed",
  "daily_rain",
  "temp_in",
  "humidity_in",
]
```

Pass multiple files or a broad glob to get a union of all populated fields across your entire dataset. The `-d 0` flag suppresses the progress line so only the TOML output appears on stdout.

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

---

### `reorganize` subcommand

Moves already-downloaded JSON weather files into a new directory structure based on the date found in each file's first record. Supports strftime tokens and `{mac}`/`{station}` placeholders — the same pattern syntax used by `ambient_download`.

```text
ambient_process reorganize [OPTIONS] <FILES>...

Arguments:
  <FILES>    Input JSON files or glob patterns

Options:
  -p, --pattern <PATTERN>    Output filename pattern [default: %Y-%m-%d.json]
  -o, --output-dir <DIR>     Base output directory [default: .]
      --mac <MAC>            MAC address for the {mac} token
      --station <STATION>    Station name for the {station} token
  -c, --config <FILE>        TOML config file (same format as ambient_download)
  -n, --dry-run              Show planned moves without executing them
```

#### Pattern tokens

| Token | Expands to |
| --- | --- |
| `%Y`, `%m`, `%d`, … | strftime date components from the file's first record |
| `{mac}` | Normalized MAC address (`AA-BB-CC-DD-EE-FF`) |
| `{station}` | Station name, or normalized MAC if no name is set |

#### Reorganize config file

Use `-c` to point at an existing `ambient_tools.toml`. The command reads `filename_pattern`, `output_folder`, `mac_address`, and `station_name` from it. CLI flags override config values when both are provided.

#### Examples

Preview what would be moved (dry run):

```shell
ambient_process reorganize --dry-run \
  --pattern '%Y/%m/%Y-%m-%d.json' \
  'data/*.json'
```

Reorganize files into year/month subdirectories:

```shell
ambient_process reorganize \
  --pattern '%Y/%m/%Y-%m-%d.json' \
  --output-dir /archive/weather \
  'data/*.json'
```

Reorganize using settings from an existing config file:

```shell
ambient_process reorganize \
  -c ambient_tools.toml \
  'data/*.json'
```

Include station name in the directory structure:

```shell
ambient_process reorganize \
  --pattern '{station}/%Y/%m/%Y-%m-%d.json' \
  --station roof \
  'data/*.json'
```

Files whose target path already exists are skipped with a warning.
