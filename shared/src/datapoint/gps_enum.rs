use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// A GPS coordinate in either decimal degrees or degrees, minutes, seconds.
/// This is useful for when you want to store a GPS coordinate in a database or other storage medium.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Gps {
    /// A GPS coordinate in decimal degrees.
    Decimal { latitude: f32, longitude: f32 },
    /// A GPS coordinate in degrees, minutes, seconds.
    Dms {
        lat_d: i16,
        lat_m: i16,
        lat_s: f32,
        lon_d: i16,
        lon_m: i16,
        lon_s: f32,
    },

    /// A GPS coordinate in degrees, decimal minutes.
    Ddm {
        lat_d: i16,
        lat_ms: f32,
        lon_d: i16,
        lon_ms: f32,
    },
}

impl Gps {
    /// Get the latitude in decimal degrees.
    pub fn latitude_decimal(&self) -> f32 {
        match self {
            Self::Decimal {
                latitude,
                longitude: _,
            } => *latitude,
            Self::Dms {
                lat_d,
                lat_m,
                lat_s,
                lon_d: _,
                lon_m: _,
                lon_s: _,
            } => dms_to_decimal(*lat_d, *lat_m, *lat_s),
            Self::Ddm {
                lat_d,
                lat_ms,
                lon_d: _,
                lon_ms: _,
            } => ddm_to_decimal(*lat_d, *lat_ms),
        }
    }

    /// Get the longitude in decimal degrees.
    pub fn longitude_decimal(&self) -> f32 {
        match self {
            Self::Decimal {
                latitude: _,
                longitude,
            } => *longitude,
            Self::Dms {
                lat_d: _,
                lat_m: _,
                lat_s: _,
                lon_d,
                lon_m,
                lon_s,
            } => dms_to_decimal(*lon_d, *lon_m, *lon_s),
            Self::Ddm {
                lat_d: _,
                lat_ms: _,
                lon_d,
                lon_ms,
            } => ddm_to_decimal(*lon_d, *lon_ms),
        }
    }

    /// Set the latitude in decimal degrees.
    ///
    /// # Arguments
    ///
    /// * `lat` - The latitude in decimal degrees.
    ///
    /// # Example
    ///
    /// ```
    /// use shared::datapoint::gps_enum::Gps;
    /// let mut gps = Gps::Decimal{ latitude: 40.7128, longitude: -74.0060 };
    /// gps.set_latitude_decimal(37.7749);
    /// assert_eq!(gps.latitude_decimal(), 37.7749);
    /// ```
    pub fn set_latitude_decimal(&mut self, lat: f32) {
        match self {
            Self::Decimal {
                latitude,
                longitude: _,
            } => *latitude = lat,
            Self::Dms {
                lat_d,
                lat_m,
                lat_s,
                lon_d: _,
                lon_m: _,
                lon_s: _,
            } => {
                (*lat_d, *lat_m, *lat_s) = decimal_to_dms(lat);
            }
            Self::Ddm {
                lat_d,
                lat_ms,
                lon_d: _,
                lon_ms: _,
            } => {
                (*lat_d, *lat_ms) = decimal_to_ddm(lat);
            }
        }
    }

    /// Set the longitude in decimal degrees.
    pub fn set_longitude_decimal(&mut self, lon: f32) {
        match self {
            Self::Decimal {
                latitude: _,
                longitude,
            } => *longitude = lon,
            Self::Dms {
                lat_d: _,
                lat_m: _,
                lat_s: _,
                lon_d,
                lon_m,
                lon_s,
            } => {
                (*lon_d, *lon_m, *lon_s) = decimal_to_dms(lon);
            }
            Self::Ddm {
                lat_d: _,
                lat_ms: _,
                lon_d,
                lon_ms,
            } => {
                (*lon_d, *lon_ms) = decimal_to_ddm(lon);
            }
        }
    }

    /// Get the latitude in degrees, minutes, seconds.
    pub fn latitude_dms(&self) -> (i16, i16, f32) {
        match self {
            Self::Decimal {
                latitude,
                longitude: _,
            } => decimal_to_dms(*latitude),
            Self::Dms {
                lat_d,
                lat_m,
                lat_s,
                lon_d: _,
                lon_m: _,
                lon_s: _,
            } => (*lat_d, *lat_m, *lat_s),
            Self::Ddm {
                lat_d,
                lat_ms,
                lon_d: _,
                lon_ms: _,
            } => ddm_to_dms(*lat_d, *lat_ms),
        }
    }

    /// Get the longitude in degrees, minutes, seconds.
    pub fn longitude_dms(&self) -> (i16, i16, f32) {
        match self {
            Self::Decimal {
                latitude: _,
                longitude,
            } => decimal_to_dms(*longitude),
            Self::Dms {
                lat_d: _,
                lat_m: _,
                lat_s: _,
                lon_d,
                lon_m,
                lon_s,
            } => (*lon_d, *lon_m, *lon_s),
            Self::Ddm {
                lat_d: _,
                lat_ms: _,
                lon_d,
                lon_ms,
            } => ddm_to_dms(*lon_d, *lon_ms),
        }
    }

    /// Set the latitude in degrees, minutes, seconds.
    pub fn set_latitude_dms(&mut self, latitude_d: i16, latitude_m: i16, latitude_s: f32) {
        match self {
            Self::Decimal {
                latitude,
                longitude: _,
            } => *latitude = dms_to_decimal(latitude_d, latitude_m, latitude_s),
            Self::Dms {
                lat_d,
                lat_m,
                lat_s,
                lon_d: _,
                lon_m: _,
                lon_s: _,
            } => {
                *lat_d = latitude_d;
                *lat_m = latitude_m;
                *lat_s = latitude_s;
            }
            Self::Ddm {
                lat_d,
                lat_ms,
                lon_d: _,
                lon_ms: _,
            } => {
                *lat_d = dms_to_ddm(latitude_d, latitude_m, latitude_s).0;
                *lat_ms = dms_to_ddm(latitude_d, latitude_m, latitude_s).1;
            }
        }
    }

    /// Set the longitude in degrees, minutes, seconds.
    pub fn set_longitude_dms(&mut self, longitude_d: i16, longitude_m: i16, longitude_s: f32) {
        match self {
            Self::Decimal {
                latitude: _,
                longitude,
            } => *longitude = dms_to_decimal(longitude_d, longitude_m, longitude_s),
            Self::Dms {
                lat_d: _,
                lat_m: _,
                lat_s: _,
                lon_d,
                lon_m,
                lon_s,
            } => {
                *lon_d = longitude_d;
                *lon_m = longitude_m;
                *lon_s = longitude_s;
            }
            Self::Ddm {
                lat_d: _,
                lat_ms: _,
                lon_d,
                lon_ms,
            } => {
                *lon_d = dms_to_ddm(longitude_d, longitude_m, longitude_s).0;
                *lon_ms = dms_to_ddm(longitude_d, longitude_m, longitude_s).1;
            }
        }
    }

    /// Get the latitude in degrees, decimal minutes.
    pub fn latitude_ddm(&self) -> (i16, f32) {
        match self {
            Self::Decimal {
                latitude,
                longitude: _,
            } => decimal_to_ddm(*latitude),
            Self::Dms {
                lat_d,
                lat_m,
                lat_s,
                lon_d: _,
                lon_m: _,
                lon_s: _,
            } => dms_to_ddm(*lat_d, *lat_m, *lat_s),
            Self::Ddm {
                lat_d,
                lat_ms,
                lon_d: _,
                lon_ms: _,
            } => (*lat_d, *lat_ms),
        }
    }

    /// Get the longitude in degrees, decimal minutes.
    pub fn longitude_ddm(&self) -> (i16, f32) {
        match self {
            Self::Decimal {
                latitude: _,
                longitude,
            } => decimal_to_ddm(*longitude),
            Self::Dms {
                lat_d: _,
                lat_m: _,
                lat_s: _,
                lon_d,
                lon_m,
                lon_s,
            } => dms_to_ddm(*lon_d, *lon_m, *lon_s),
            Self::Ddm {
                lat_d: _,
                lat_ms: _,
                lon_d,
                lon_ms,
            } => (*lon_d, *lon_ms),
        }
    }

    /// Set the latitude in degrees, decimal minutes.
    pub fn set_latitude_ddm(&mut self, latitude_d: i16, latitude_m: f32) {
        match self {
            Self::Decimal {
                latitude,
                longitude: _,
            } => *latitude = ddm_to_decimal(latitude_d, latitude_m),
            Self::Dms {
                lat_d,
                lat_m,
                lat_s,
                lon_d: _,
                lon_m: _,
                lon_s: _,
            } => {
                *lat_d = ddm_to_dms(latitude_d, latitude_m).0;
                *lat_m = ddm_to_dms(latitude_d, latitude_m).1;
                *lat_s = 0.0;
            }
            Self::Ddm {
                lat_d,
                lat_ms,
                lon_d: _,
                lon_ms: _,
            } => {
                *lat_d = latitude_d;
                *lat_ms = latitude_m;
            }
        }
    }

    /// Set the longitude in degrees, decimal minutes.
    pub fn set_longitude_ddm(&mut self, longitude_d: i16, longitude_m: f32) {
        match self {
            Self::Decimal {
                latitude: _,
                longitude,
            } => *longitude = ddm_to_decimal(longitude_d, longitude_m),
            Self::Dms {
                lat_d: _,
                lat_m: _,
                lat_s: _,
                lon_d,
                lon_m,
                lon_s,
            } => {
                *lon_d = ddm_to_dms(longitude_d, longitude_m).0;
                *lon_m = ddm_to_dms(longitude_d, longitude_m).1;
                *lon_s = 0.0;
            }
            Self::Ddm {
                lat_d: _,
                lat_ms: _,
                lon_d,
                lon_ms,
            } => {
                *lon_d = longitude_d;
                *lon_ms = longitude_m;
            }
        }
    }

    /// Set the format to decimal degrees.
    pub fn switch_to_decimal(&mut self) {
        match self {
            Self::Decimal {
                latitude: _,
                longitude: _,
            } => (),
            Self::Dms {
                lat_d,
                lat_m,
                lat_s,
                lon_d,
                lon_m,
                lon_s,
            } => {
                let lat = dms_to_decimal(*lat_d, *lat_m, *lat_s);
                let lon = dms_to_decimal(*lon_d, *lon_m, *lon_s);
                *self = Self::Decimal {
                    latitude: lat,
                    longitude: lon,
                };
            }
            Self::Ddm {
                lat_d,
                lat_ms,
                lon_d,
                lon_ms,
            } => {
                let lat = ddm_to_decimal(*lat_d, *lat_ms);
                let lon = ddm_to_decimal(*lon_d, *lon_ms);
                *self = Self::Decimal {
                    latitude: lat,
                    longitude: lon,
                };
            }
        }
    }

    /// Set the format to degrees, minutes, seconds.
    pub fn switch_to_dms(&mut self) {
        match self {
            Self::Decimal {
                latitude,
                longitude,
            } => {
                let (lat_d, lat_m, lat_s) = decimal_to_dms(*latitude);
                let (lon_d, lon_m, lon_s) = decimal_to_dms(*longitude);
                *self = Self::Dms {
                    lat_d,
                    lat_m,
                    lat_s,
                    lon_d,
                    lon_m,
                    lon_s,
                };
            }
            Self::Dms {
                lat_d: _,
                lat_m: _,
                lat_s: _,
                lon_d: _,
                lon_m: _,
                lon_s: _,
            } => (),
            Self::Ddm {
                lat_d,
                lat_ms,
                lon_d,
                lon_ms,
            } => {
                let (lat_d, lat_m, lat_s) = ddm_to_dms(*lat_d, *lat_ms);
                let (lon_d, lon_m, lon_s) = ddm_to_dms(*lon_d, *lon_ms);
                *self = Self::Dms {
                    lat_d,
                    lat_m,
                    lat_s,
                    lon_d,
                    lon_m,
                    lon_s,
                };
            }
        }
    }

    /// Set the format to degrees, decimal minutes.
    pub fn switch_to_ddm(&mut self) {
        match self {
            Self::Decimal {
                latitude,
                longitude,
            } => {
                let (lat_d, lat_ms) = decimal_to_ddm(*latitude);
                let (lon_d, lon_ms) = decimal_to_ddm(*longitude);
                *self = Self::Ddm {
                    lat_d,
                    lat_ms,
                    lon_d,
                    lon_ms,
                };
            }
            Self::Dms {
                lat_d,
                lat_m,
                lat_s,
                lon_d,
                lon_m,
                lon_s,
            } => {
                let (lat_d, lat_ms) = dms_to_ddm(*lat_d, *lat_m, *lat_s);
                let (lon_d, lon_ms) = dms_to_ddm(*lon_d, *lon_m, *lon_s);
                *self = Self::Ddm {
                    lat_d,
                    lat_ms,
                    lon_d,
                    lon_ms,
                };
            }
            Self::Ddm {
                lat_d: _,
                lat_ms: _,
                lon_d: _,
                lon_ms: _,
            } => (),
        }
    }
}

impl Display for Gps {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decimal {
                latitude,
                longitude,
            } => write!(f, "({}, {})", latitude, longitude),
            Self::Dms {
                lat_d,
                lat_m,
                lat_s,
                lon_d,
                lon_m,
                lon_s,
            } => {
                let ns = if *lat_d < 0 { 'S' } else { 'N' };
                let ew = if *lon_d < 0 { 'W' } else { 'E' };
                write!(
                    f,
                    "({}°{}'{}\"{ns}, {}°{}'{}\"{ew})",
                    lat_d.abs(),
                    lat_m,
                    lat_s,
                    lon_d.abs(),
                    lon_m,
                    lon_s
                )
            }
            Self::Ddm {
                lat_d,
                lat_ms,
                lon_d,
                lon_ms,
            } => {
                let ns = if *lat_d < 0 { 'S' } else { 'N' };
                let ew = if *lon_d < 0 { 'W' } else { 'E' };
                write!(
                    f,
                    "({}°{}'{ns}, {}°{}'{ew})",
                    lat_d.abs(),
                    lat_ms,
                    lon_d.abs(),
                    lon_ms
                )
            }
        }
    }
}

/// Convert degrees, minutes, seconds to decimal degrees.
fn dms_to_decimal(degrees: i16, minutes: i16, seconds: f32) -> f32 {
    f32::from(degrees) + f32::from(minutes) / 60.0 + seconds / 3600.0
}

/// Convert degees, decimal minutes to decimal degrees.
fn ddm_to_decimal(degrees: i16, minutes: f32) -> f32 {
    // Convert the minutes to decimal.
    let m = minutes / 60.0;

    if degrees < 0 {
        f32::from(degrees) - m
    } else {
        f32::from(degrees) + m
    }
}

/// Convert decimal degrees to degrees, minutes, seconds.
fn decimal_to_dms(decimal: f32) -> (i16, i16, f32) {
    let degrees = (decimal.trunc() % 180.0) as i16;
    let minutes = (decimal.abs() - decimal.abs().trunc()) * 60.0;
    let seconds = minutes.fract() * 60.0;
    (degrees, minutes as i16, seconds)
}

/// Convert decimal degrees to degrees, decimal minutes.
fn decimal_to_ddm(decimal: f32) -> (i16, f32) {
    let degrees = (decimal.trunc() % 180.0) as i16;
    let minutes = decimal.fract().abs() * 60.0;
    (degrees, minutes)
}

/// convert degrees, decimal minutes to degrees, minutes, seconds.
fn ddm_to_dms(degs: i16, mins: f32) -> (i16, i16, f32) {
    let minutes = mins as i16;
    let seconds = (mins.fract()) * 60.0;
    (degs, minutes, seconds)
}

/// Convert degrees, minutes, seconds to degrees, decimal minutes.
fn dms_to_ddm(degrees: i16, minutes: i16, seconds: f32) -> (i16, f32) {
    let minutes = f32::from(minutes) + seconds / 60.0;
    (degrees, minutes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_to_dms() {
        let (d, m, s) = decimal_to_dms(40.7128);
        assert_eq!(d, 40);
        assert_eq!(m, 42);
        assert_eq!(s, 46.07666);
    }

    #[test]
    fn test_dms_to_decimal() {
        let decimal = dms_to_decimal(40, 42, 46.08);
        assert_eq!(decimal, 40.7128);
    }

    #[test]
    fn test_decimal_to_ddm() {
        let (d, m) = decimal_to_ddm(-74.0060);
        assert_eq!(d, -74);
        assert_eq!(m, 0.35980225);
    }

    #[test]
    fn test_ddm_to_decimal() {
        let decimal = ddm_to_decimal(-74, 0.36);
        assert_eq!(decimal, -74.0060);
    }

    #[test]
    fn test_dms_to_ddm() {
        let (d, m) = dms_to_ddm(40, 42, 46.08);
        assert_eq!(d, 40);
        assert_eq!(m, 42.768);
    }

    #[test]
    fn test_ddm_to_dms() {
        let (d, m, s) = ddm_to_dms(-74, 0.36);
        assert_eq!(d, -74);
        assert_eq!(m, 0);
        assert_eq!(s, 21.6);
    }

    #[test]
    fn test_gps_coordinate() {
        let mut gps = Gps::Decimal {
            latitude: 40.7128,
            longitude: -74.0060,
        };
        assert_eq!(gps.latitude_decimal(), 40.7128);
        assert_eq!(gps.longitude_decimal(), -74.0060);

        gps.set_latitude_decimal(37.7749);
        assert_eq!(gps.latitude_decimal(), 37.7749);

        gps.set_longitude_decimal(-122.4194);
        assert_eq!(gps.longitude_decimal(), -122.4194);

        let (lat_d, lat_m, lat_s) = gps.latitude_dms();
        assert_eq!(lat_d, 37);
        assert_eq!(lat_m, 46);
        assert_eq!(lat_s, 29.634705);

        let (lon_d, lon_m, lon_s) = gps.longitude_dms();
        assert_eq!(lon_d, -122);
        assert_eq!(lon_m, 25);
        assert_eq!(lon_s, 9.851074);

        gps.set_latitude_dms(37, 46, 29.64);
        assert_eq!(gps.latitude_decimal(), 37.7749);
        assert_eq!(gps.longitude_decimal(), -122.4194);
        assert_eq!(gps.latitude_ddm(), (37, 46.49391));
    }

    #[test]
    fn test_latitude_decimal() {
        let gps = Gps::Decimal {
            latitude: 40.7128,
            longitude: -74.0060,
        };
        assert_eq!(gps.latitude_decimal(), 40.7128);

        let gps = Gps::Dms {
            lat_d: 37,
            lat_m: 46,
            lat_s: 29.634705,
            lon_d: -122,
            lon_m: 25,
            lon_s: 9.851074,
        };
        assert_eq!(gps.latitude_decimal(), 37.7749);

        let gps = Gps::Ddm {
            lat_d: 37,
            lat_ms: 46.64,
            lon_d: -122,
            lon_ms: 25.15,
        };
        assert_eq!(gps.latitude_decimal(), 37.777332);
    }

    #[test]
    fn test_longitude_decimal() {
        let gps = Gps::Decimal {
            latitude: 40.7128,
            longitude: -74.0060,
        };
        assert_eq!(gps.longitude_decimal(), -74.0060);
    }

    #[test]
    fn test_set_latitude_decimal() {
        let mut gps = Gps::Decimal {
            latitude: 40.7128,
            longitude: -74.0060,
        };
        gps.set_latitude_decimal(37.7749);
        assert_eq!(gps.latitude_decimal(), 37.7749);
    }

    #[test]
    fn test_set_longitude_decimal() {
        let mut gps = Gps::Decimal {
            latitude: 40.7128,
            longitude: -74.0060,
        };
        gps.set_longitude_decimal(-122.4194);
        assert_eq!(gps.longitude_decimal(), -122.4194);
    }

    #[test]
    fn test_latitude_dms() {
        let gps = Gps::Dms {
            lat_d: 37,
            lat_m: 46,
            lat_s: 29.634705,
            lon_d: -122,
            lon_m: 25,
            lon_s: 9.851074,
        };
        assert_eq!(gps.latitude_dms(), (37, 46, 29.634705));
    }

    #[test]
    fn test_longitude_dms() {
        let gps = Gps::Dms {
            lat_d: 37,
            lat_m: 46,
            lat_s: 29.634705,
            lon_d: -122,
            lon_m: 25,
            lon_s: 9.851074,
        };
        assert_eq!(gps.longitude_dms(), (-122, 25, 9.851074));
    }

    #[test]
    fn test_set_latitude_dms() {
        let mut gps = Gps::Dms {
            lat_d: 37,
            lat_m: 46,
            lat_s: 29.634705,
            lon_d: -122,
            lon_m: 25,
            lon_s: 9.851074,
        };
        gps.set_latitude_dms(37, 46, 29.64);
        assert_eq!(gps.latitude_dms(), (37, 46, 29.64));
    }

    #[test]
    fn test_set_longitude_dms() {
        let mut gps = Gps::Dms {
            lat_d: 37,
            lat_m: 46,
            lat_s: 29.634705,
            lon_d: -122,
            lon_m: 25,
            lon_s: 9.851074,
        };
        gps.set_longitude_dms(-122, 25, 10.0);
        assert_eq!(gps.longitude_dms(), (-122, 25, 10.0));
    }

    #[test]
    fn test_latitude_ddm() {
        let gps = Gps::Ddm {
            lat_d: 37,
            lat_ms: 46.64,
            lon_d: -122,
            lon_ms: 25.15,
        };
        assert_eq!(gps.latitude_ddm(), (37, 46.64));
    }

    #[test]
    fn test_longitude_ddm() {
        let gps = Gps::Ddm {
            lat_d: 37,
            lat_ms: 46.64,
            lon_d: -122,
            lon_ms: 25.15,
        };
        assert_eq!(gps.longitude_ddm(), (-122, 25.15));
    }

    #[test]
    fn test_set_latitude_ddm() {
        let mut gps = Gps::Ddm {
            lat_d: 37,
            lat_ms: 46.64,
            lon_d: -122,
            lon_ms: 25.15,
        };
        gps.set_latitude_ddm(37, 46.49391);
        assert_eq!(gps.latitude_ddm(), (37, 46.49391));
    }

    #[test]
    fn test_set_longitude_ddm() {
        let mut gps = Gps::Ddm {
            lat_d: 37,
            lat_ms: 46.64,
            lon_d: -122,
            lon_ms: 25.15,
        };
        gps.set_longitude_ddm(-122, 25.25);
        assert_eq!(gps.longitude_ddm(), (-122, 25.25));
    }

    #[test]
    fn test_switch_to_decimal() {
        let mut gps = Gps::Dms {
            lat_d: 37,
            lat_m: 46,
            lat_s: 29.634705,
            lon_d: -122,
            lon_m: 25,
            lon_s: 9.851074,
        };
        gps.switch_to_decimal();
        assert_eq!(
            gps,
            Gps::Decimal {
                latitude: 37.7749,
                longitude: -121.5806,
            }
        );
    }

    #[test]
    fn test_switch_to_dms() {
        let mut gps = Gps::Decimal {
            latitude: 37.7749,
            longitude: -122.4194,
        };
        gps.switch_to_dms();
        assert_eq!(
            gps,
            Gps::Dms {
                lat_d: 37,
                lat_m: 46,
                lat_s: 29.634705,
                lon_d: -122,
                lon_m: 25,
                lon_s: 9.851074,
            }
        );
    }

    #[test]
    fn test_switch_to_ddm() {
        let mut gps = Gps::Dms {
            lat_d: 37,
            lat_m: 46,
            lat_s: 29.634705,
            lon_d: -122,
            lon_m: 25,
            lon_s: 9.851074,
        };
        gps.switch_to_ddm();
        assert_eq!(
            gps,
            Gps::Ddm {
                lat_d: 37,
                lat_ms: 46.49391,
                lon_d: -122,
                lon_ms: 25.164185,
            }
        );
    }

    #[test]
    fn test_display() {
        let gps = Gps::Decimal {
            latitude: 37.7749,
            longitude: -122.4194,
        };
        assert_eq!(format!("{gps}"), "(37.7749, -122.4194)");

        let gps = Gps::Dms {
            lat_d: 37,
            lat_m: 46,
            lat_s: 29.634705,
            lon_d: -122,
            lon_m: 25,
            lon_s: 9.851074,
        };
        assert_eq!(format!("{gps}"), "(37°46'29.634705\"N, 122°25'9.851074\"W)");

        let gps = Gps::Ddm {
            lat_d: 37,
            lat_ms: 46.49391,
            lon_d: -122,
            lon_ms: 25.164185,
        };
        assert_eq!(format!("{gps}"), "(37°46.49391'N, 122°25.164185'W)");
    }
}
