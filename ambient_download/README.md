# ambient_download

Downloads weather and device data from the [Ambient Weather API](https://ambientweather.docs.apiary.io/) and writes it to JSON files. Designed for scheduled daily collection or one-off historical backfills.

## Table of Contents

- [ambient\_download](#ambient_download)
  - [Table of Contents](#table-of-contents)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Global options](#global-options)
    - [`device` subcommand](#device-subcommand)
    - [`weather` subcommand](#weather-subcommand)
      - [Sleep time and API rate limiting](#sleep-time-and-api-rate-limiting)
    - [`timezone` subcommand](#timezone-subcommand)
    - [`newconfig` subcommand](#newconfig-subcommand)
  - [Config file](#config-file)
  - [Filename patterns](#filename-patterns)
  - [Environment variables](#environment-variables)
  - [Scheduling](#scheduling)
    - [cron (Linux / macOS)](#cron-linux--macos)
    - [Task Scheduler (Windows)](#task-scheduler-windows)
  - [Security](#security)

---

## Prerequisites

- An Ambient Weather account with an API key and an Application key.
  - Sign up at [ambientweather.net](https://ambientweather.net/).
  - Key management: <https://ambientweather.com/faqs/question/view/id/1834/>
- The MAC address of your weather station console. You can find it on the device itself, in the Ambient Weather dashboard, or by running the `device` subcommand once your keys are configured.

---

## Installation

No binary releases are available yet. Build from source using Cargo:

```shell
git clone https://github.com/evensolberg/ambient_tools.git
cd ambient_tools
cargo build --release
```

The compiled binary is placed at `target/release/ambient_download`.

---

## Usage

```text
ambient_download [OPTIONS] [COMMAND]

Options:
  -d, --detail-level <detail>    Output detail level: 0=Quiet 1=Normal 2=Detailed 3=Debug 4=Trace
                                 [default: 1] [env: AMBIENT_WEATHER_DETAIL_LEVEL]
  -k, --api-key                  Ambient Weather API key
                                 [env: AMBIENT_WEATHER_API_KEY]
  -l, --app-key                  Ambient Weather Application key
                                 [env: AMBIENT_WEATHER_APP_KEY]
  -o, --output-folder            Output folder [default: .] [env: AMBIENT_WEATHER_OUTPUT_FOLDER]
  -c, --config-file              TOML config file. When provided, replaces all other options.

Commands:
  device    Download device information
  weather   Download weather information
  timezone  Print timezone info; optionally convert a datetime between UTC and local
  newconfig Create a new config file template
  help      Print this message or the help of a subcommand
```

### Global options

| Flag | Short | Description |
 ---|---|---|
| `--detail-level` | `-d` | Verbosity: 0 silent, 1 normal, 2 detailed, 3 debug, 4 trace |
| `--api-key` | `-k` | Ambient Weather API key |
| `--app-key` | `-l` | Ambient Weather Application key |
| `--output-folder` | `-o` | Directory for all output files |
| `--config-file` | `-c` | Path to a TOML config file. Overrides all other flags when supplied. |

Credentials and the output folder can be supplied as environment variables instead of flags. See [Environment variables](#environment-variables).

---

### `device` subcommand

Downloads device information for all stations associated with your account and writes it to a JSON file. The output includes MAC addresses, station names, and location metadata. Run this once to discover the MAC address you need for the `weather` subcommand.

```text
ambient_download device [OPTIONS]

Options:
  -f, --device-info-filename    Output filename [default: device-info.json]
      --save-mac                Write the discovered MAC address back to the config file specified
                                by --config-file. If multiple devices are found, the first MAC is
                                saved and the rest are appended as commented-out lines.
```

Example:

```shell
ambient_download \
  --api-key YOUR_API_KEY \
  --app-key YOUR_APP_KEY \
  device --device-info-filename my-devices.json
```

---

### `weather` subcommand

Downloads weather records for one or more days and writes each day to a separate JSON file.

```text
ambient_download weather [OPTIONS] [DATE]

Arguments:
  [DATE]    Start date in YYYY-MM-DD format. Defaults to yesterday if omitted.

Options:
  -m, --mac-address        Device MAC address [env: AMBIENT_WEATHER_MAC_ADDRESS]
  -n, --limit <limit>      Records to download per day, max 288 [default: 288]
                           [env: AMBIENT_WEATHER_LIMIT]
  -d, --days <days>        Number of days to download, max 1095 [default: 1]
                           [env: AMBIENT_WEATHER_DAYS]
  -z, --tz-name            IANA timezone name [env: AMBIENT_WEATHER_TZ_NAME]
  -s, --sleep-time         Seconds to sleep between daily requests [default: 10]
                           [env: AMBIENT_WEATHER_SLEEP_TIME]
  -p, --filename-pattern   strftime pattern for output filenames. Supports {mac} and {station}
                           tokens. [default: %Y-%m-%d.json]
                           [env: AMBIENT_WEATHER_FILENAME_PATTERN]
      --station-name       Human-readable name used for the {station} token in filename patterns.
                           Falls back to a normalized MAC address if not set.
                           [env: AMBIENT_WEATHER_STATION_NAME]
```

Example — download yesterday's data:

```shell
ambient_download \
  --api-key YOUR_API_KEY \
  --app-key YOUR_APP_KEY \
  weather --mac-address AA:BB:CC:DD:EE:FF
```

Example — download the last 7 days starting from a specific date:

```shell
ambient_download \
  --api-key YOUR_API_KEY \
  --app-key YOUR_APP_KEY \
  weather --mac-address AA:BB:CC:DD:EE:FF --days 7 2024-05-01
```

#### Sleep time and API rate limiting

The Ambient Weather API enforces rate limits. When `--sleep-time` is left at its default, the tool automatically scales the delay between requests based on the number of days being downloaded:

| Days | Sleep between requests |
|---|---|
| 1–5 | 10 seconds |
| 6–15 | 30 seconds |
| 16–30 | 60 seconds |
| 31–90 | 120 seconds |
| 91+ | 300 seconds |

Supply `--sleep-time` explicitly to override this behaviour.

---

### `timezone` subcommand

Prints the system's local timezone information. With an optional datetime argument, converts that datetime between UTC and local time in both directions.

```text
ambient_download timezone [DATETIME]

Arguments:
  [DATETIME]  Datetime to convert, in YYYY-MM-DD HH:MM:SS format (optional)
```

Without an argument, prints local time, UTC time, UTC offset, and the IANA timezone name.

With an argument, prints both the UTC-to-local and local-to-UTC conversions for that datetime.

Example:

```shell
ambient_download timezone "2024-05-01 08:00:00"
```

---

### `newconfig` subcommand

Generates a TOML config file template with all available fields pre-populated. The timezone is auto-detected from the system.

```text
ambient_download newconfig [FILENAME]

Arguments:
  [FILENAME]  Path for the new config file [default: ambient_download.toml]
```

Example:

```shell
ambient_download newconfig /etc/ambient_download.toml
```

---

## Config file

A config file replaces all command-line flags. Pass it with `--config-file`:

```shell
ambient_download --config-file /path/to/ambient_download.toml weather
```

Generate a template with `newconfig`, then fill in your credentials:

```toml
app_key = ""
api_key = ""
mac_address = ""
output_folder = "."
filename_pattern = "%Y-%m-%d.json"
station_name = ""
tz_name = "America/Vancouver"   # auto-detected from system; change as needed
detail_level = 1
limit = 288
sleep_time = 10
```

All fields are optional within the file; omitted fields fall back to their defaults. Store the file outside your project directory and restrict its permissions (see [Security](#security)).

---

## Filename patterns

The `--filename-pattern` option (and its config-file equivalent) accepts any [strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) format string, plus two custom tokens:

| Token | Replaced with |
|---|---|
| `{mac}` | The device MAC address, with colons replaced by hyphens |
| `{station}` | The value of `--station-name`, or the normalized MAC if not set |

Intermediate directories implied by the pattern are created automatically.

Examples:

| Pattern | Resulting filename |
|---|---|
| `%Y-%m-%d.json` (default) | `2024-05-01.json` |
| `%Y/%Y-%m-%d.json` | `2024/2024-05-01.json` |
| `{station}/%Y-%m-%d.json` | `home/2024-05-01.json` |
| `%Y/%m/{mac}-%Y-%m-%d.json` | `2024/05/AA-BB-CC-DD-EE-FF-2024-05-01.json` |

All output files are written as pretty-printed (indented) JSON.

---

## Environment variables

All credentials and common settings can be supplied as environment variables, making it straightforward to configure the tool in containers or CI environments without a config file.

| Variable | Description |
|---|---|
| `AMBIENT_WEATHER_API_KEY` | Ambient Weather API key |
| `AMBIENT_WEATHER_APP_KEY` | Ambient Weather Application key |
| `AMBIENT_WEATHER_MAC_ADDRESS` | Device MAC address |
| `AMBIENT_WEATHER_TZ_NAME` | IANA timezone name (e.g. `America/New_York`) |
| `AMBIENT_WEATHER_OUTPUT_FOLDER` | Directory for output files |
| `AMBIENT_WEATHER_DETAIL_LEVEL` | Verbosity level, 0–4 |
| `AMBIENT_WEATHER_LIMIT` | Records to download per day (max 288) |
| `AMBIENT_WEATHER_DAYS` | Number of days to download |
| `AMBIENT_WEATHER_SLEEP_TIME` | Seconds to sleep between daily requests |
| `AMBIENT_WEATHER_FILENAME_PATTERN` | strftime output filename pattern |
| `AMBIENT_WEATHER_STATION_NAME` | Station name used for the `{station}` token |

Command-line flags take precedence over environment variables. A config file supplied via `--config-file` overrides both.

---

## Scheduling

The tool is designed to run unattended as a scheduled task. The recommended approach is to create a config file with `newconfig`, populate it with your credentials, and invoke the binary with `--config-file`.

### cron (Linux / macOS)

Add a line to your crontab (`crontab -e`) to download the previous day's data every day at 01:00:

```text
0 1 * * * /path/to/ambient_download --config-file /path/to/ambient_download.toml weather
```

To download the previous day's data and organize files by year and month:

```text
0 1 * * * /path/to/ambient_download --config-file /path/to/ambient_download.toml weather --filename-pattern '%Y/%m/%Y-%m-%d.json'
```

### Task Scheduler (Windows)

Use the [Windows Task Scheduler](https://learn.microsoft.com/en-us/windows/win32/taskschd/about-the-task-scheduler) to run the binary on a daily trigger with the same arguments.

---

## Security

- **Credentials in config files.** Config files contain API keys and Application keys in plaintext. Restrict permissions and keep the file out of version control:

  ```shell
  chmod 600 /path/to/ambient_download.toml
  echo "ambient_download.toml" >> .gitignore
  ```

- **Credential redaction in logs.** API keys and Application keys are redacted from all log output and serialized data regardless of the detail level selected.

- **HTTP log suppression.** Internal HTTP client logs (reqwest/hyper) are suppressed. The Ambient Weather API embeds credentials as URL query parameters; allowing those logs through would expose your keys.
