pub mod battery;
pub mod direction;
pub mod download;
pub mod leak;
pub mod length;
pub mod pressure;
pub mod speed;
pub mod temperature;

use battery::BatteryStatus;
use direction::WindDirection;
use leak::LeakDetector;
use length::Length;
use pressure::AirPressure;
use speed::WindSpeed;
use temperature::Temperature;

use chrono::{DateTime, FixedOffset, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
struct WeatherDataPoint {
    /// milliseconds from 1970-01-01 rounded down to nearest minute on server
    pub dateutc: Option<u64>,

    /// Indoor temperature in degrees Fahrenheit
    pub tempinf: Option<Temperature>,

    /// Indoor battery OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub battin: Option<BatteryStatus>,

    /// Inndoor humidity in percent (0-100%)
    pub humidityin: Option<u8>,

    /// Relative pressure indoors (inHg)
    pub baromrelin: Option<AirPressure>,

    /// Absolutie pressure indoors (inHg)
    pub barmabsin: Option<AirPressure>,

    /// Outdoor battery OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub battout: Option<BatteryStatus>,

    /// Rain sensor battery OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    ///
    /// NOTE: This is returned as a string and must be converted to an integer.
    pub battrain: Option<BatteryStatus>,

    /// Instantaneous wind direction (0-360º)
    pub winddir: Option<WindDirection>,

    /// Average wind direction, 2 minute average (0-360º)
    pub winddir_avg2m: Option<WindDirection>,

    /// Average wind direction, 10 minute average (0-360º)
    pub winddir_avg10m: Option<WindDirection>,

    /// Wind Guest direction (0-360º)
    pub windgustdir: Option<WindDirection>,

    /// Windgust WindSpeed (mph)
    pub windgustmph: Option<WindSpeed>,

    /// Maximum wind WindSpeed during the last day (mph)
    pub maxdailygust: Option<WindSpeed>,

    /// Wind speed (mph)
    windspeedmph: Option<WindSpeed>,

    /// Wind speed, 2 minute average (mph)
    windspdmph_avg2m: Option<WindSpeed>,

    /// Wind speed, 10 minute average (mph)
    windspdmph_avg10m: Option<WindSpeed>,

    /// Hourly Rain Rate (inches/hour)
    pub hourlyrainin: Option<Length>,

    /// Event rain (inches)
    pub eventrainin: Option<Length>,

    /// Daily rain (inches)
    pub dailyrainin: Option<Length>,

    /// Weekly rain (inches)
    pub weeklyrainin: Option<Length>,

    /// Monthly rain (inches)
    pub monthlyrainin: Option<Length>,

    /// Yearly rain (inches)
    pub yearlyrainin: Option<Length>,

    /// Total rain since last factory reset (inches)
    pub totalrainin: Option<Length>,

    /// Solar radiation (W/m^2)
    pub solarradiation: Option<f32>,

    /// UV Index
    pub uv: Option<u8>,

    /// Temperature 1 in degrees Fahrenheit
    pub temp1f: Option<Temperature>,

    /// Humidity 1 in percent (0-100%)
    pub humidity1: Option<u8>,

    /// Temperature 2 in degrees Fahrenheit
    pub temp2f: Option<Temperature>,

    /// Humidity 2 in percent (0-100%)
    pub humidity2: Option<u8>,

    /// Temperature 3 in degrees Fahrenheit
    pub temp3f: Option<Temperature>,

    /// Humidity 3 in percent (0-100%)
    pub humidity3: Option<u8>,

    /// Temperature 4 in degrees Fahrenheit
    pub temp4f: Option<Temperature>,

    /// Humidity 4 in percent (0-100%)
    pub humidity4: Option<u8>,

    /// Temperature 5 in degrees Fahrenheit
    pub temp5f: Option<Temperature>,

    /// Humidity 5 in percent (0-100%)
    pub humidity5: Option<u8>,

    /// Temperature 6 in degrees Fahrenheit
    pub temp6f: Option<Temperature>,

    /// Humidity 6 in percent (0-100%)
    pub humidity6: Option<u8>,

    /// Temperature 7 in degrees Fahrenheit
    pub temp7f: Option<Temperature>,

    /// Humidity 7 in percent (0-100%)
    pub humidity7: Option<u8>,

    /// Temperature 8 in degrees Fahrenheit
    pub temp8f: Option<Temperature>,

    /// Humidity 8 in percent (0-100%)
    pub humidity8: Option<u8>,

    /// Temperature 9 in degrees Fahrenheit
    pub temp9f: Option<Temperature>,

    /// Humidity 9 in percent (0-100%)
    pub humidity9: Option<u8>,

    /// Temperature 10 in degrees Fahrenheit
    pub temp10f: Option<Temperature>,

    /// Humidity 10 in percent (0-100%)
    pub humidity10: Option<u8>,

    /// Battery 1 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt1: Option<BatteryStatus>,

    /// Battery 2 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt2: Option<BatteryStatus>,

    /// Battery 3 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt3: Option<BatteryStatus>,

    /// Battery 4 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt4: Option<BatteryStatus>,

    /// Battery 5 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt5: Option<BatteryStatus>,

    /// Battery 6 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt6: Option<BatteryStatus>,

    /// Battery 7 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt7: Option<BatteryStatus>,

    /// Battery 8 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt8: Option<BatteryStatus>,

    /// Battery 9 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt9: Option<BatteryStatus>,

    /// Battery 10 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt10: Option<BatteryStatus>,

    /// CO2 battery OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt_co2: Option<BatteryStatus>,

    /// Feels Like indicator for sensor 1 (F, calculated on server)
    pub feelsLike1f: Option<Temperature>,

    /// Dew Point indicator for sensor 1 (F, calculated on server)
    pub dewPoint1f: Option<Temperature>,

    /// Feels Like indicator for sensor 2 (F, calculated on server)
    pub feelsLike2f: Option<Temperature>,

    /// Dew Point indicator for sensor 2 (F, calculated on server)
    pub dewPoint2f: Option<Temperature>,

    /// Feels Like indicator for sensor 3 (F, calculated on server)
    pub feelsLike3f: Option<Temperature>,

    /// Dew Point indicator for sensor 3 (F, calculated on server)
    pub dewPoint3f: Option<Temperature>,

    /// Feels Like indicator for sensor 4 (F, calculated on server)
    pub feelsLike4f: Option<Temperature>,

    /// Dew Point indicator for sensor 4 (F, calculated on server)
    pub dewPoint4f: Option<Temperature>,

    /// Feels Like indicator for sensor 5 (F, calculated on server)
    pub feelsLike5f: Option<Temperature>,

    /// Dew Point indicator for sensor 5 (F, calculated on server)
    pub dewPoint5f: Option<Temperature>,

    /// Feels Like indicator for sensor 6 (F, calculated on server)
    pub feelsLike6f: Option<Temperature>,

    /// Dew Point indicator for sensor 6 (F, calculated on server)
    pub dewPoint6f: Option<Temperature>,

    /// Feels Like indicator for sensor 7 (F, calculated on server)
    pub feelsLike7f: Option<Temperature>,

    /// Dew Point indicator for sensor 7 (F, calculated on server)
    pub dewPoint7f: Option<Temperature>,

    /// Feels Like indicator for sensor 8 (F, calculated on server)
    pub feelsLike8f: Option<Temperature>,

    /// Dew Point indicator for sensor 8 (F, calculated on server)
    pub dewPoint8f: Option<Temperature>,

    /// Feels Like indicator for sensor 9 (F, calculated on server)
    pub feelsLike9f: Option<Temperature>,

    /// Dew Point indicator for sensor 9 (F, calculated on server)
    pub dewPoint9f: Option<Temperature>,

    /// Feels Like indicator for sensor 10 (F, calculated on server)
    pub feelsLike10f: Option<Temperature>,

    /// Dew Point indicator for sensor 10 (F, calculated on server)
    pub dewPoint10f: Option<Temperature>,

    /// Feeks Like indicator for indoor sensor (F, calculated on server)
    pub feelsLikein: Option<Temperature>,

    /// Dew Point indicator for indoor sensor (F, calculated on server)
    pub dewPointin: Option<Temperature>,

    /// Last time hourlyrainin > 0 (UTC, calculated on server)
    pub lastRain: Option<DateTime<Local>>,

    /// Local time of last update
    pub date: Option<DateTime<Local>>,

    /// 24 hour rain (inches) (field: `24hourrainin` - Rust doesn't allow numbers at the beginning of field names)
    pub hour24rainin: Option<Length>,

    /// AQI (Air Quality Index) derived from PM10 indoor - 24 hour running average
    pub aqi_pm10_24h_aqin: Option<u8>,

    /// AQI (Air Quality Index) derived from PM10 indoor
    pub aqi_pm10_aqin: Option<u8>,

    /// AQI (Air Quality Index) derived from PM2.5 indoor - 24 hour running average
    pub aqi_pm25_24h_aqin: Option<u8>,

    /// AQI (Air Quality Index) derived from PM2.5 outdoor
    pub aqi_pm25_aqin: Option<u8>,

    /// AQI derived from PM25 IN
    pub aqi_pm25_in: Option<u8>,

    /// AQI derived from PM25 IN - 24 hour running average
    pub aqi_pm25_in_24h: Option<u8>,

    /// Leak detector battery 1 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt_leak1: Option<BatteryStatus>,

    /// Leak detector battery 2 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt_leak2: Option<BatteryStatus>,

    /// Leak detector battery 3 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt_leak3: Option<BatteryStatus>,

    /// Leak detector battery 4 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt_leak4: Option<BatteryStatus>,

    /// PM2.5 Air Quality Sensor Battery indication OK/Low indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    pub batt_pm25: Option<BatteryStatus>,

    /// Cellular gateway battery OK/Low Indication - 1=OK, 0=Low
    pub batt_cellgateway: Option<BatteryStatus>,

    /// Lightning detector battery OK/Low Indication - 1 = Low, 0 = OK
    pub batt_lightning: Option<BatteryStatus>,

    /// Soil moisture sensor 1 battery OK/Low Indication - 0 = Low, 1 = OK
    pub batt_sm1: Option<BatteryStatus>,

    /// Soil moisture sensor 2 battery OK/Low Indication - 0 = Low, 1 = OK
    pub batt_sm2: Option<BatteryStatus>,

    /// Soil moisture sensor 3 battery OK/Low Indication - 0 = Low, 1 = OK
    pub batt_sm3: Option<BatteryStatus>,

    /// Soil moisture sensor 4 battery OK/Low Indication - 0 = Low, 1 = OK
    pub batt_sm4: Option<BatteryStatus>,

    /// CO2 measurement in ppm
    pub co2: Option<u16>,

    /// Indoor CO2 from AQIN - 24 hour running average (ppm)
    pub co2_in_24h_aqin: Option<u16>,

    /// Indoor CO2 from AQIN (ppm)
    pub co2_in_aqin: Option<u16>,

    /// [Evapotranspiration](https://en.wikipedia.org/wiki/Evapotranspiration) short (inches/day)
    pub etos: Option<Length>,

    /// [Evapotranspiration](https://en.wikipedia.org/wiki/Evapotranspiration) tall (inches/day)
    pub etrs: Option<Length>,

    /// [Growing Degree Days](https://en.wikipedia.org/wiki/Growing_degree-day)
    pub gdd: Option<u16>,

    /// Outdoor humidity in percent (0-100%)
    pub humidity: Option<u8>,

    /// Leaf wetness sensor 1 (0-100%)
    pub leafwetness1: Option<u8>,

    /// Leaf wetness sensor 2 (0-100%)
    pub leafwetness2: Option<u8>,

    /// Leaf wetness sensor 3 (0-100%)
    pub leafwetness3: Option<u8>,

    /// Leaf wetness sensor 4 (0-100%)
    pub leafwetness4: Option<u8>,

    /// Leaf wetness sensor 5 (0-100%)
    pub leafwetness5: Option<u8>,

    /// Leaf wetness sensor 6 (0-100%)
    pub leafwetness6: Option<u8>,

    /// Leaf wetness sensor 7 (0-100%)
    pub leafwetness7: Option<u8>,

    /// Leaf wetness sensor 8 (0-100%)
    pub leafwetness8: Option<u8>,

    /// Leak detector 1 status - 0 = OK, 1 = Leak, 2 = Offline, 100 = Unknown
    pub leak1: Option<LeakDetector>,

    /// Leak detector 2 status - 0 = OK, 1 = Leak, 2 = Offline, 100 = Unknown
    pub leak2: Option<LeakDetector>,

    /// Leak detector 3 status - 0 = OK, 1 = Leak, 2 = Offline, 100 = Unknown
    pub leak3: Option<LeakDetector>,

    /// Leak detector 4 status - 0 = OK, 1 = Leak, 2 = Offline, 100 = Unknown
    pub leak4: Option<LeakDetector>,

    /// Lightning strikes per day
    pub lightning_day: Option<u16>,

    /// Lightning distance in miles
    pub lightning_distance: Option<f32>,

    /// Lightning strikes per hour
    pub lightning_hour: Option<u16>,

    /// Last lightning strike time
    pub lightning_time: Option<DateTime<Local>>,

    /// PM10 Air Quality Sensor - 24 hour running average (ug/m^3)
    pub pm10_in_24h_aqin: Option<f32>,

    /// PM10 Air Quality Sensor (ug/m^3)
    pub pm10_in_aqin: Option<f32>,

    /// PM2.5 Air Quality (ug/m^3)
    pub pm25: Option<f32>,

    /// PM2.5 Air Quality 24 hour average (ug/m^3)
    pub pm25_24h: Option<f32>,

    /// PM2.5 Air Quality - Indoor (ug/m^3)
    pub pm25_in: Option<f32>,

    /// PM2.5 Air Quality 24 hour average - Indoor (ug/m^3)
    pub pm25_in_24h: Option<f32>,

    /// PM2.5 Air Quality Sensor indoor - 24 hour running average - AQIN sensor (ug/m^3)
    pub pm25_in_24h_aqin: Option<f32>,

    /// PM2.5 Air Quality Sensor indoor - AQIN sensor (ug/m^3)
    pub pm25_in_aqin: Option<f32>,

    /// Indoor PM sensor humidity
    pub pm_in_humidity_aqin: Option<u8>,

    /// Indoor PM sensor temperature (F)
    pub pm_in_temp_aqin: Option<Temperature>,

    /// Relay 1 status - 0 = Off, 1 = On
    pub relay1: Option<u8>,

    /// Relay 2 status - 0 = Off, 1 = On
    pub relay2: Option<u8>,

    /// Relay 3 status - 0 = Off, 1 = On
    pub relay3: Option<u8>,

    /// Relay 4 status - 0 = Off, 1 = On
    pub relay4: Option<u8>,

    /// Relay 5 status - 0 = Off, 1 = On
    pub relay5: Option<u8>,

    /// Relay 6 status - 0 = Off, 1 = On
    pub relay6: Option<u8>,

    /// Relay 7 status - 0 = Off, 1 = On
    pub relay7: Option<u8>,

    /// Relay 8 status - 0 = Off, 1 = On
    pub relay8: Option<u8>,

    /// Relay 9 status - 0 = Off, 1 = On
    pub relay9: Option<u8>,

    /// Relay 10 status - 0 = Off, 1 = On
    pub relay10: Option<u8>,

    /// Soil Humidity 1 (0-100.0%)
    pub soilhum1: Option<f32>,

    /// Soil Humidity 2 (0-100.0%)
    pub soilhum2: Option<f32>,

    /// Soil Humidity 3 (0-100.0%)
    pub soilhum3: Option<f32>,

    /// Soil Humidity 4 (0-100.0%)
    pub soilhum4: Option<f32>,

    /// Soil Humidity 5 (0-100.0%)
    pub soilhum5: Option<f32>,

    /// Soil Humidity 6 (0-100.0%)
    pub soilhum6: Option<f32>,

    /// Soil Humidity 7 (0-100.0%)
    pub soilhum7: Option<f32>,

    /// Soil Humidity 8 (0-100.0%)
    pub soilhum8: Option<f32>,

    /// Soil Humidity 9 (0-100.0%)
    pub soilhum9: Option<f32>,

    /// Soil Humidity 10 (0-100.0%)
    pub soilhum10: Option<f32>,

    /// Soil Temperature 1 (F)
    pub soiltemp1: Option<Temperature>,

    /// Soil Temperature 2 (F)
    pub soiltemp2: Option<Temperature>,

    /// Soil Temperature 3 (F)
    pub soiltemp3: Option<Temperature>,

    /// Soil Temperature 4 (F)
    pub soiltemp4: Option<Temperature>,

    /// Soil Temperature 5 (F)
    pub soiltemp5: Option<Temperature>,

    /// Soil Temperature 6 (F)
    pub soiltemp6: Option<Temperature>,

    /// Soil Temperature 7 (F)
    pub soiltemp7: Option<Temperature>,

    /// Soil Temperature 8 (F)
    pub soiltemp8: Option<Temperature>,

    /// Soil Temperature 9 (F)
    pub soiltemp9: Option<Temperature>,

    /// Soil Temperature 10 (F)
    pub soiltemp10: Option<Temperature>,

    /// Soil tension 1 (centibar - cb)
    pub soil_tension1: Option<f32>,

    /// Soil tension 2 (centibar - cb)
    pub soil_tension2: Option<f32>,

    /// Soil tension 3 (centibar - cb)
    pub soil_tension3: Option<f32>,

    /// Soil tension 4 (centibar - cb)
    pub soil_tension4: Option<f32>,

    /// Outdoor temperature in degrees Fahrenheit
    pub tempf: Option<Temperature>,

    /// IANA Timezone for the station
    pub tz: Option<String>,
}
