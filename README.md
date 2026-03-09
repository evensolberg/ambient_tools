# ambient_tools

A Rust workspace for downloading and working with data from the [Ambient Weather API](https://ambientweather.docs.apiary.io/).

## Tools

| Tool | Description |
| --- | --- |
| [ambient_download](ambient_download/README.md) | Downloads weather and device data from the Ambient Weather API. Designed for scheduled or one-off use. |
| [ambient_process](ambient_process/README.md) | Reads downloaded JSON files and converts them to CSV with optional date filtering and unit conversion. Also pretty-prints JSON files in place. |
