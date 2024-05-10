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

### Environment Variables

The Ambient Download application supports the following environment variables:

- `AMBIENT_WEATHER_API_KEY`: The Ambient Weather API key.
- `AMBIENT_WEATHER_APP_KEY`: The Ambient Weather Application key.
- `AMBIENT_WEATHER_MAC_ADDRESS`: The MAC address of the Ambient Weather device to download data from.
- `AMBIENT_WEATHER_TZ_OFFSET`: The time zone offset in hours from UTC. Default is 0.
- `AMBIENT_WEATHER_TIMEZONE`: The time zone name. Default is "UTC".
- `AMBIENT_WEATHER_OUTPUT_FOLDER`: The folder into which the output files are to be written. Default is the current folder if not specified.
- `AMBIENT_WEATHER_DETAIL_LEVEL`: Output detail level. 0=Quiet, 1=Normal, 2=Detailed, 3=Debug, 4=Trace.
-
