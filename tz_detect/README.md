# `tz_detect` - Timezone detection for the Ambient Weather downloader

This is a little utility that attempts to detect the local time zone and offset from UTC for the Ambient Weather downloader.

## Usage

```text
Usage: tz_detect [OPTIONS]

Options:
  -c, --config-file <CONFIG_FILE>  The configuration file to write the offset information into
  -h, --help                       Print help
  -V, --version                    Print version
```

## Configuration File

The configuration file is a simple TOML file that contains the following information:

```toml
api_key = "<your api key>"
app_key = "<your app key"
mac_address = "<your console's MAC address>"
output_folder = "<the folder to write the data to>"
tz_offset = "<the timezone offset from UTC>" # This utility can update this value.
detail_level = <0-4> # 0 = Quiet, 1 = Normal, 2 = Detailed, 3 = Debug, 4 = Trace
limit = 288 # the number of records to download - 288 is the maximum (5 minute intervals for 24 hours)
```

If the file doesn't exist, it will be created with default values. If the file does exist, the utility will update the `tz_offset` value with the detected offset.

### Usage

```shell
tz_detect -c /path/to/config.toml
```
