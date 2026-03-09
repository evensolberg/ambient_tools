use super::battery::BatteryStatus;
use super::direction::WindDirection;
use super::leak::LeakDetector;
use super::length::Length;
use super::pressure::AirPressure;
use super::speed::WindSpeed;
use super::temperature::Temperature;
use super::units::SystemOfUnits;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherDataPoint {
    /// The unit of measure
    pub measurement_system: Option<SystemOfUnits>,

    /// milliseconds from 1970-01-01 rounded down to nearest minute on server
    #[serde(rename(deserialize = "dateutc"))]
    pub dateutc: Option<u64>,

    /// Indoor temperature in degrees Fahrenheit
    #[serde(rename(deserialize = "tempinf"))]
    pub temp: Option<Temperature>,

    /// Indoor battery OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "battin"))]
    pub indoor_battery: Option<BatteryStatus>,

    /// Inndoor humidity in percent (0-100%)
    #[serde(rename(deserialize = "humidityin"))]
    pub indoor_humidity_percent: Option<u8>,

    /// Relative pressure indoors (inHg)
    #[serde(rename(deserialize = "baromrelin"))]
    pub indoor_relative_pressure: Option<AirPressure>,

    /// Absolutie pressure indoors (inHg)
    #[serde(rename(deserialize = "baromabsin"))]
    pub indoor_absolute_pressure: Option<AirPressure>,

    /// Outdoor battery OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "battout"))]
    pub outdoor_battery_status: Option<BatteryStatus>,

    /// Rain sensor battery OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    ///
    /// NOTE: This is returned as a string and must be converted to an integer.
    #[serde(rename(deserialize = "battrain"))]
    pub rain_sensor_battery_status: Option<BatteryStatus>,

    /// Instantaneous wind direction (0-360º)
    #[serde(rename(deserialize = "winddir"))]
    pub wind_direction: Option<WindDirection>,

    /// Average wind direction, 2 minute average (0-360º)
    #[serde(rename(deserialize = "winddir_avg2m"))]
    pub wind_direction_2min_average: Option<WindDirection>,

    /// Average wind direction, 10 minute average (0-360º)
    #[serde(rename(deserialize = "winddir_avg10m"))]
    pub wind_direction_10min_average: Option<WindDirection>,

    /// Wind Guest direction (0-360º)
    #[serde(rename(deserialize = "windgustdir"))]
    pub wind_gust_direction: Option<WindDirection>,

    /// Windgust `WindSpeed` (mph)
    #[serde(rename(deserialize = "windgustmph"))]
    pub wind_gust_speed: Option<WindSpeed>,

    /// Maximum wind `WindSpeed` during the last day (mph)
    #[serde(rename(deserialize = "maxdailygust"))]
    pub wind_gust_speed_daily_max: Option<WindSpeed>,

    /// Wind speed (mph)
    #[serde(rename(deserialize = "windspeedmph"))]
    pub wind_speed: Option<WindSpeed>,

    /// Wind speed, 2 minute average (mph)
    #[serde(rename(deserialize = "windspdmph_avg2m"))]
    pub wind_speed_2min_average: Option<WindSpeed>,

    /// Wind speed, 10 minute average (mph)
    #[serde(rename(deserialize = "windspdmph_avg10m"))]
    pub wind_speed_10min_average: Option<WindSpeed>,

    /// Hourly Rain Rate (inches/hour)
    #[serde(rename(deserialize = "hourlyrainin"))]
    pub rain_hourly_rate: Option<Length>,

    /// Event rain (inches)
    #[serde(rename(deserialize = "eventrainin"))]
    pub rain_event_amount: Option<Length>,

    /// Daily rain (inches)
    #[serde(rename(deserialize = "dailyrainin"))]
    pub rain_daily: Option<Length>,

    /// Weekly rain (inches)
    #[serde(rename(deserialize = "weeklyrainin"))]
    pub rain_weekly: Option<Length>,

    /// Monthly rain (inches)
    #[serde(rename(deserialize = "monthlyrainin"))]
    pub rain_monthly: Option<Length>,

    /// Yearly rain (inches)
    #[serde(rename(deserialize = "yearlyrainin"))]
    pub rain_yearly: Option<Length>,

    /// Total rain since last factory reset (inches)
    #[serde(rename(deserialize = "totalrainin"))]
    pub rain_total: Option<Length>,

    /// Solar radiation (W/m^2)
    #[serde(rename(deserialize = "solarradiation"))]
    pub solar_radiation: Option<f32>,

    /// UV Index
    #[serde(rename(deserialize = "uv"))]
    pub uv_index: Option<u8>,

    /// Temperature 1 in degrees Fahrenheit
    #[serde(rename(deserialize = "temp1f"))]
    pub temp_1: Option<Temperature>,

    /// Humidity 1 in percent (0-100%)
    #[serde(rename(deserialize = "humidity1"))]
    pub humidity_1: Option<u8>,

    /// Temperature 2 in degrees Fahrenheit
    #[serde(rename(deserialize = "temp2f"))]
    pub temp_2: Option<Temperature>,

    /// Humidity 2 in percent (0-100%)
    #[serde(rename(deserialize = "humidity2"))]
    pub humidity_2: Option<u8>,

    /// Temperature 3 in degrees Fahrenheit
    #[serde(rename(deserialize = "temp3f"))]
    pub temp_3: Option<Temperature>,

    /// Humidity 3 in percent (0-100%)
    #[serde(rename(deserialize = "humidity3"))]
    pub humidity_3: Option<u8>,

    /// Temperature 4 in degrees Fahrenheit
    #[serde(rename(deserialize = "temp4f"))]
    pub temp_4: Option<Temperature>,

    /// Humidity 4 in percent (0-100%)
    #[serde(rename(deserialize = "humidity4"))]
    pub humidity_4: Option<u8>,

    /// Temperature 5 in degrees Fahrenheit
    #[serde(rename(deserialize = "temp5f"))]
    pub temp_5: Option<Temperature>,

    /// Humidity 5 in percent (0-100%)
    #[serde(rename(deserialize = "humidity5"))]
    pub humidity_5: Option<u8>,

    /// Temperature 6 in degrees Fahrenheit
    #[serde(rename(deserialize = "temp6f"))]
    pub temp_6: Option<Temperature>,

    /// Humidity 6 in percent (0-100%)
    #[serde(rename(deserialize = "humidity6"))]
    pub humidity_6: Option<u8>,

    /// Temperature 7 in degrees Fahrenheit
    #[serde(rename(deserialize = "temp7f"))]
    pub temp_7: Option<Temperature>,

    /// Humidity 7 in percent (0-100%)
    #[serde(rename(deserialize = "humidity7"))]
    pub humidity_7: Option<u8>,

    /// Temperature 8 in degrees Fahrenheit
    #[serde(rename(deserialize = "temp8f"))]
    pub temp_8: Option<Temperature>,

    /// Humidity 8 in percent (0-100%)
    #[serde(rename(deserialize = "humidity8"))]
    pub humidity_8: Option<u8>,

    /// Temperature 9 in degrees Fahrenheit
    #[serde(rename(deserialize = "temp9f"))]
    pub temp_9: Option<Temperature>,

    /// Humidity 9 in percent (0-100%)
    #[serde(rename(deserialize = "humidity9"))]
    pub humidity_9: Option<u8>,

    /// Temperature 10 in degrees Fahrenheit
    #[serde(rename(deserialize = "temp10f"))]
    pub temp_10: Option<Temperature>,

    /// Humidity 10 in percent (0-100%)
    #[serde(rename(deserialize = "humidity10"))]
    pub humidity_10: Option<u8>,

    /// Battery 1 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt1"))]
    pub battery_1_status: Option<BatteryStatus>,

    /// Battery 2 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt2"))]
    pub battery_2_status: Option<BatteryStatus>,

    /// Battery 3 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt3"))]
    pub battery_3_status: Option<BatteryStatus>,

    /// Battery 4 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt4"))]
    pub battery_4_status: Option<BatteryStatus>,

    /// Battery 5 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt5"))]
    pub battery_5_status: Option<BatteryStatus>,

    /// Battery 6 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt6"))]
    pub battery_6_status: Option<BatteryStatus>,

    /// Battery 7 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt7"))]
    pub battery_7_status: Option<BatteryStatus>,

    /// Battery 8 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt8"))]
    pub battery_8_status: Option<BatteryStatus>,

    /// Battery 9 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt9"))]
    pub battery_9_status: Option<BatteryStatus>,

    /// Battery 10 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt10"))]
    pub battery_10_status: Option<BatteryStatus>,

    /// CO2 battery OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt_co2"))]
    pub battery_co2_status: Option<BatteryStatus>,

    /// Feels Like indicator for sensor 1 (F, calculated on server)
    #[serde(rename(deserialize = "feelsLike1f"))]
    pub feels_like_1: Option<Temperature>,

    /// Dew Point indicator for sensor 1 (F, calculated on server)
    #[serde(rename(deserialize = "dewPoint1f"))]
    pub dew_point_1: Option<Temperature>,

    /// Feels Like indicator for sensor 2 (F, calculated on server)
    #[serde(rename(deserialize = "feelsLike2f"))]
    pub feels_like_2: Option<Temperature>,

    /// Dew Point indicator for sensor 2 (F, calculated on server)
    #[serde(rename(deserialize = "dewPoint2f"))]
    pub dew_point_2: Option<Temperature>,

    /// Feels Like indicator for sensor 3 (F, calculated on server)
    #[serde(rename(deserialize = "feelsLike3f"))]
    pub feels_like_3: Option<Temperature>,

    /// Dew Point indicator for sensor 3 (F, calculated on server)
    #[serde(rename(deserialize = "dewPoint3f"))]
    pub dew_point_3: Option<Temperature>,

    /// Feels Like indicator for sensor 4 (F, calculated on server)
    #[serde(rename(deserialize = "feelsLike4f"))]
    pub feels_like_4: Option<Temperature>,

    /// Dew Point indicator for sensor 4 (F, calculated on server)
    #[serde(rename(deserialize = "dewPoint4f"))]
    pub dew_point_4: Option<Temperature>,

    /// Feels Like indicator for sensor 5 (F, calculated on server)
    #[serde(rename(deserialize = "feelsLike5f"))]
    pub feels_like_5: Option<Temperature>,

    /// Dew Point indicator for sensor 5 (F, calculated on server)
    #[serde(rename(deserialize = "dewPoint5f"))]
    pub dew_point_5: Option<Temperature>,

    /// Feels Like indicator for sensor 6 (F, calculated on server)
    #[serde(rename(deserialize = "feelsLike6f"))]
    pub feels_like_6: Option<Temperature>,

    /// Dew Point indicator for sensor 6 (F, calculated on server)
    #[serde(rename(deserialize = "dewPoint6f"))]
    pub dew_point_6: Option<Temperature>,

    /// Feels Like indicator for sensor 7 (F, calculated on server)
    #[serde(rename(deserialize = "feelsLike7f"))]
    pub feels_like_7: Option<Temperature>,

    /// Dew Point indicator for sensor 7 (F, calculated on server)
    #[serde(rename(deserialize = "dewPoint7f"))]
    pub dew_point_7: Option<Temperature>,

    /// Feels Like indicator for sensor 8 (F, calculated on server)
    #[serde(rename(deserialize = "feelsLike8f"))]
    pub feels_like_8: Option<Temperature>,

    /// Dew Point indicator for sensor 8 (F, calculated on server)
    #[serde(rename(deserialize = "dewPoint8f"))]
    pub dew_point_8: Option<Temperature>,

    /// Feels Like indicator for sensor 9 (F, calculated on server)
    #[serde(rename(deserialize = "feelsLike9f"))]
    pub feels_like_9: Option<Temperature>,

    /// Dew Point indicator for sensor 9 (F, calculated on server)
    #[serde(rename(deserialize = "dewPoint9f"))]
    pub dew_point_9: Option<Temperature>,

    /// Feels Like indicator for sensor 10 (F, calculated on server)
    #[serde(rename(deserialize = "feelsLike10f"))]
    pub feels_like_10: Option<Temperature>,

    /// Dew Point indicator for sensor 10 (F, calculated on server)
    #[serde(rename(deserialize = "dewPoint10f"))]
    pub dew_point_10: Option<Temperature>,

    /// Feeks Like indicator for indoor sensor (F, calculated on server)
    #[serde(rename(deserialize = "feelsLikein"))]
    pub feels_like_inside: Option<Temperature>,

    /// Dew Point indicator for indoor sensor (F, calculated on server)
    #[serde(rename(deserialize = "dewPointin"))]
    pub dew_point_inside: Option<Temperature>,

    /// Last time hourlyrainin > 0 (UTC, calculated on server)
    #[serde(rename(deserialize = "lastRain"))]
    pub last_rain: Option<DateTime<Local>>,

    /// Local time of last update
    #[serde(rename(deserialize = "date"))]
    pub last_updated: Option<DateTime<Local>>,

    /// 24 hour rain (inches) (field: `24hourrainin` - Rust doesn't allow numbers at the beginning of field names)
    #[serde(rename(deserialize = "24hourrainin"))]
    pub last_24_hour_rain: Option<Length>,

    /// AQI (Air Quality Index) derived from PM10 - 24 hour running average
    #[serde(rename(deserialize = "aqi_pm10_24h_aqin"))]
    pub air_quality_index_pm10_24h: Option<u8>,

    /// AQI (Air Quality Index) derived from PM10
    #[serde(rename(deserialize = "aqi_pm10_aqin"))]
    pub air_quality_index_pm10_indoor: Option<u8>,

    /// AQI (Air Quality Index) derived from PM2.5 - 24 hour running average
    #[serde(rename(deserialize = "aqi_pm25_24h_aqin"))]
    pub air_quality_index_pm25_24h: Option<u8>,

    /// AQI (Air Quality Index) derived from PM2.5 indoor
    #[serde(rename(deserialize = "aqi_pm25_aqin"))]
    pub air_quality_index_pm25: Option<u8>,

    /// AQI derived from PM25 indoor
    #[serde(rename(deserialize = "aqi_pm25_in"))]
    pub air_quality_index_pm25_indoor: Option<u8>,

    /// AQI derived from PM25 IN - 24 hour running average
    #[serde(rename(deserialize = "aqi_pm25_in_24h"))]
    pub air_quality_index_pm25_24h_indoor: Option<u8>,

    /// Leak detector battery 1 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt_leak1"))]
    pub battery_leak1: Option<BatteryStatus>,

    /// Leak detector battery 2 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt_leak2"))]
    pub battery_leak2: Option<BatteryStatus>,

    /// Leak detector battery 3 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt_leak3"))]
    pub battery_leak3: Option<BatteryStatus>,

    /// Leak detector battery 4 OK/Low Indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt_leak4"))]
    pub battery_leak4: Option<BatteryStatus>,

    /// PM2.5 Air Quality Sensor Battery indication OK/Low indication - 1=OK, 0=Low (Meteobridge Users: 1=Low, 0=OK)
    #[serde(rename(deserialize = "batt_pm25"))]
    pub battery_pm25: Option<BatteryStatus>,

    /// Cellular gateway battery OK/Low Indication - 1=OK, 0=Low
    #[serde(rename(deserialize = "batt_cellgateway"))]
    pub battery_cellgateway: Option<BatteryStatus>,

    /// Lightning detector battery OK/Low Indication - 1 = Low, 0 = OK
    #[serde(rename(deserialize = "batt_lightning"))]
    pub battery_lightning: Option<BatteryStatus>,

    /// Soil moisture sensor 1 battery OK/Low Indication - 0 = Low, 1 = OK
    #[serde(rename(deserialize = "batt_sm1"))]
    pub battery_soil_moisture_1: Option<BatteryStatus>,

    /// Soil moisture sensor 2 battery OK/Low Indication - 0 = Low, 1 = OK
    #[serde(rename(deserialize = "batt_sm2"))]
    pub battery_soil_moisture_2: Option<BatteryStatus>,

    /// Soil moisture sensor 3 battery OK/Low Indication - 0 = Low, 1 = OK
    #[serde(rename(deserialize = "batt_sm3"))]
    pub battery_soil_moisture_3: Option<BatteryStatus>,

    /// Soil moisture sensor 4 battery OK/Low Indication - 0 = Low, 1 = OK
    #[serde(rename(deserialize = "batt_sm4"))]
    pub battery_soil_moisture_4: Option<BatteryStatus>,

    /// CO2 measurement in ppm
    #[serde(rename(deserialize = "co2"))]
    pub co2_ppm: Option<u16>,

    /// Indoor CO2 from AQIN - 24 hour running average (ppm)
    #[serde(rename(deserialize = "co2_in_24h_aqin"))]
    pub air_quality_index_co2_24h_indoor: Option<u16>,

    /// Indoor CO2 from AQIN (ppm)
    #[serde(rename(deserialize = "co2_in_aqin"))]
    pub air_quality_index_co2_indoor: Option<u16>,

    /// [Evapotranspiration](https://en.wikipedia.org/wiki/Evapotranspiration) short (inches/day)
    #[serde(rename(deserialize = "etos"))]
    pub evapotranspiration_short: Option<Length>,

    /// [Evapotranspiration](https://en.wikipedia.org/wiki/Evapotranspiration) tall (inches/day)
    #[serde(rename(deserialize = "etrs"))]
    pub evapotranspiration_tall: Option<Length>,

    /// [Growing Degree Days](https://en.wikipedia.org/wiki/Growing_degree-day)
    #[serde(rename(deserialize = "gdd"))]
    pub growing_degree_days: Option<u16>,

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
    #[serde(rename(deserialize = "soilhum1"))]
    pub soil_humidity_1: Option<f32>,

    /// Soil Humidity 2 (0-100.0%)
    #[serde(rename(deserialize = "soilhum2"))]
    pub soil_humidity_2: Option<f32>,

    /// Soil Humidity 3 (0-100.0%)
    #[serde(rename(deserialize = "soilhum3"))]
    pub soil_humidity_3: Option<f32>,

    /// Soil Humidity 4 (0-100.0%)
    #[serde(rename(deserialize = "soilhum4"))]
    pub soil_humidity_4: Option<f32>,

    /// Soil Humidity 5 (0-100.0%)
    #[serde(rename(deserialize = "soilhum5"))]
    pub soil_humidity_5: Option<f32>,

    /// Soil Humidity 6 (0-100.0%)
    #[serde(rename(deserialize = "soilhum6"))]
    pub soil_humidity_6: Option<f32>,

    /// Soil Humidity 7 (0-100.0%)
    #[serde(rename(deserialize = "soilhum7"))]
    pub soil_humidity_7: Option<f32>,

    /// Soil Humidity 8 (0-100.0%)
    #[serde(rename(deserialize = "soilhum8"))]
    pub soil_humidity_8: Option<f32>,

    /// Soil Humidity 9 (0-100.0%)
    #[serde(rename(deserialize = "soilhum9"))]
    pub soil_humidity_9: Option<f32>,

    /// Soil Humidity 10 (0-100.0%)
    #[serde(rename(deserialize = "soilhum10"))]
    pub soil_humidity_10: Option<f32>,

    /// Soil Temperature 1 (F)
    #[serde(rename(deserialize = "soiltemp1"))]
    pub soil_temperature_1: Option<Temperature>,

    /// Soil Temperature 2 (F)
    #[serde(rename(deserialize = "soiltemp2"))]
    pub soil_temperature_2: Option<Temperature>,

    /// Soil Temperature 3 (F)
    #[serde(rename(deserialize = "soiltemp3"))]
    pub soil_temperature_3: Option<Temperature>,

    /// Soil Temperature 4 (F)
    #[serde(rename(deserialize = "soiltemp4"))]
    pub soil_temperature_4: Option<Temperature>,

    /// Soil Temperature 5 (F)
    #[serde(rename(deserialize = "soiltemp5"))]
    pub soil_temperature_5: Option<Temperature>,

    /// Soil Temperature 6 (F)
    #[serde(rename(deserialize = "soiltemp6"))]
    pub soil_temperature_6: Option<Temperature>,

    /// Soil Temperature 7 (F)
    #[serde(rename(deserialize = "soiltemp7"))]
    pub soil_temperature_7: Option<Temperature>,

    /// Soil Temperature 8 (F)
    #[serde(rename(deserialize = "soiltemp8"))]
    pub soil_temperature_8: Option<Temperature>,

    /// Soil Temperature 9 (F)
    #[serde(rename(deserialize = "soiltemp9"))]
    pub soil_temperature_9: Option<Temperature>,

    /// Soil Temperature 10 (F)
    #[serde(rename(deserialize = "soiltemp10"))]
    pub soil_temperature_10: Option<Temperature>,

    /// Soil tension 1 (centibar - cb)
    #[serde(rename(deserialize = "soil_tension1"))]
    pub soil_tension_1: Option<f32>,

    /// Soil tension 2 (centibar - cb)
    #[serde(rename(deserialize = "soil_tension2"))]
    pub soil_tension_2: Option<f32>,

    /// Soil tension 3 (centibar - cb)
    #[serde(rename(deserialize = "soil_tension3"))]
    pub soil_tension_3: Option<f32>,

    /// Soil tension 4 (centibar - cb)
    #[serde(rename(deserialize = "soil_tension4"))]
    pub soil_tension_4: Option<f32>,

    /// Outdoor temperature in degrees Fahrenheit
    #[serde(rename(deserialize = "tempf"))]
    pub temperature: Option<Temperature>,

    /// IANA Timezone for the station
    #[serde(rename(deserialize = "tz"))]
    pub timezone: Option<String>,

    pub passkey: Option<String>,
    pub time: Option<DateTime<Local>>,
    pub loc: Option<String>,
    pub date: Option<DateTime<Local>>,
}
