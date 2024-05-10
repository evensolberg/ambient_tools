# ambient_download

Download data from the Ambient Weather API:

- GET devices: <https://rt.ambientweather.net/v1/devices?applicationKey=&apiKey=">
  - `applicationKey (String)`: Application Key
  - `apiKey (String)` - API Key for user account
- GET data: <https://rt.ambientweather.net/v1/devices/macAddress>
  - `macAddress (String)`: The device Mac Address
  - `apiKey (String)`: API Key for user account
  - `applicationKey (String)`: Application Key
  - `endDate (String)`: The most recent datetime. Results descend from there. If left blank, the most recent results will be returned. Date format should be in milliseconds since the epoch or string representations outlined here: <https://momentjs.com/docs/#/parsing/string/>. Note: datetimes are stored in UTC.
  - `limit (Number)`: The maximum number of results to return (max: 288) Default: 288.
