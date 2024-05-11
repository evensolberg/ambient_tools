# Ambient Download

Download data from the Ambient Weather API.

This directory contains the main application for downloading data from the Ambient Weather API. The application is written in Rust and is intended to be run as a cron job to periodically download data from the API.

## Usage

```text
Usage: ambient_download [OPTIONS] --api-key --app-key [COMMAND]

Commands:
  device   Download device information.
  weather  Download weather information.
  help     Print this message or the help of the given subcommand(s)

Options:
  -d, --detail-level <detail>
          Output detail level. 0=Quiet, 1=Normal, 2=Detailed, 3=Debug, 4=Trace.
          [default: 1]

  -k, --api-key
          The Ambient Weather API key. If not provided, the value is read from the environment variable AMBIENT_WEATHER_API_KEY.

          [env: AMBIENT_WEATHER_API_KEY=<YOUR KEY>]

  -l, --app-key
          The Ambient Weather Application key. If not provided, the value is read from the environment variable AMBIENT_WEATHER_APP_KEY.

          [env: AMBIENT_WEATHER_APP_KEY=<YOUR KEY>]

  -o, --output-folder <output-folder>
          The folder into which the output files are to be written. Default is the current folder if not specified.

          [default: .]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

Note that all of the following commands require the `--api-key` and `--app-key` options to be specified. These options can also be set using the `AMBIENT_WEATHER_API_KEY` and `AMBIENT_WEATHER_APP_KEY` environment variables.

### `device` Command

Invoke the `device` command to download device information. The device information is written to a file in JSON format.

Usage information:

```text
Usage: ambient_download --api-key --app-key device [OPTIONS]

Options:
  -f, --device-info-filename [<device-info-filename>...]
          The name of the file into which the device information is to be written. Default is 'device-info.json' if not specified.

  -h, --help
          Print help (see a summary with '-h')
```

Example:

```shell
ambient_download --api-key <YOUR_API_KEY> --app-key <YOUR_APP_KEY> device
```

This will produce a JSON file named `device-info.json` in the current folder. The file contains information about the Ambient Weather device,
including the MAC address, name, location, and other details. You will need the MAC address to download weather data.

### `weather` Command

Invoke the `weather` command to download weather information. The weather information is written to a file in JSON format.

Usage information:

```text
Usage: ambient_download --api-key --app-key weather [OPTIONS] --mac-address [DATES]...

Arguments:
  [DATES]...
          One or more dates(s), If nothing is entered, yesterday's date is used.

Options:
  -m, --mac-address
          The MAC address for the console for which we are downloading information. If not provided, the value is read from the environment variable AMBIENT_WEATHER_MAC_ADDRESS.

          [env: AMBIENT_WEATHER_MAC_ADDRESS=C4:5B:BE:5F:08:EE]

  -n, --limit <limit>
          The number of records to download. The maximum is 288.

          [default: 288]

  -z, --time-zone
          The time zone for the data. If not provided, the value is read from the environment variable AMBIENT_WEATHER_TIME_ZONE.

          [env: AMBIENT_WEATHER_TIME_ZONE=]

  -o, --tz-offset <tz-offset>
          The time zone offset for the data. If not provided, the value is read from the environment variable AMBIENT_WEATHER_TZ_OFFSET.

          [env: AMBIENT_WEATHER_TZ_OFFSET=-07:00]

  -h, --help
          Print help (see a summary with '-h')
```

Example:

```shell
ambient_download --api-key <YOUR_API_KEY> --app-key <YOUR_APP_KEY> weather --mac-address <YOUR_MAC_ADDRESS>
```

This will produce a JSON file named `<yesterday's date in YYYY-MM-DD format>.json` (e.g., `2024-05-30.json`) in the current folder. The file contains weather information for the specified date.

### Environment Variables

The Ambient Download application supports the following environment variables:

- `AMBIENT_WEATHER_API_KEY`: The Ambient Weather API key.
- `AMBIENT_WEATHER_APP_KEY`: The Ambient Weather Application key.
- `AMBIENT_WEATHER_MAC_ADDRESS`: The MAC address of the Ambient Weather device to download data from.
- `AMBIENT_WEATHER_TZ_OFFSET`: The time zone offset from UTC. Default is 00:00. Example: `-07:00` for Pacific Daylight Time.
- `AMBIENT_WEATHER_TIMEZONE`: The time zone name. Default is "UTC". *Not currently used.*
- `AMBIENT_WEATHER_OUTPUT_FOLDER`: The folder into which the output files are to be written. Default is the current folder if not specified.
- `AMBIENT_WEATHER_DETAIL_LEVEL`: Output detail level. *0=Quiet*, *1=Normal*, *2=Detailed*, *3=Debug*, *4=Trace*.
