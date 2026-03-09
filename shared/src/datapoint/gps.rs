//! Unit for keeping track of GPS data.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum GpsUnits {
    #[default]
    Decimal,
    DMS,
}

impl Display for GpsUnits {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Decimal => write!(f, "Decimal"),
            Self::DMS => write!(f, "DMS"),
        }
    }
}

/// A GPS coordinate. This is a simple struct that holds a latitude and longitude in decimal degrees.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
struct GpsCoordinate {
    format: GpsUnits,
    latitude: f32,
    longitude: f32,
}

#[allow(dead_code)]
impl GpsCoordinate {
    /// Create a new `GPS_Coordinate` from a latitude and longitude.
    pub const fn new(latitude: f32, longitude: f32) -> Self {
        Self {
            format: GpsUnits::Decimal,
            latitude,
            longitude,
        }
    }

    /// Get the latitude.
    pub const fn latitude_decimal(&self) -> f32 {
        self.latitude
    }

    /// Get the longitude.
    pub const fn longitude_decimal(&self) -> f32 {
        self.longitude
    }

    /// Set the latitude.
    pub const fn set_latitude_decimal(&mut self, latitude: f32) {
        self.latitude = latitude;
    }

    /// Set the longitude.
    pub const fn set_longitude_decimal(&mut self, longitude: f32) {
        self.longitude = longitude;
    }

    /// Get the format of the GPS coordinate.
    pub const fn format(&self) -> GpsUnits {
        self.format
    }

    /// Set the format of the GPS coordinate.
    pub const fn set_format(&mut self, format: GpsUnits) {
        self.format = format;
    }

    /// Get the latitude from DMS (degrees, minutes, seconds) format.
    #[allow(clippy::cast_possible_truncation)]
    pub fn latitude_dms(&self) -> (i16, i16, f32) {
        let degrees = self.latitude.abs() as i16;
        let minutes = ((self.latitude.abs() - f32::from(degrees)) * 60.0) as i16;
        let seconds =
            (self.latitude.abs() - f32::from(degrees) - f32::from(minutes) / 60.0) * 3600.0;
        (degrees, minutes, seconds)
    }

    /// Get the longitude from DMS (degrees, minutes, seconds) format.
    #[allow(clippy::cast_possible_truncation)]
    pub fn longitude_dms(&self) -> (i16, i16, f32) {
        let degrees = self.longitude.abs() as i16;
        let minutes = ((self.longitude.abs() - f32::from(degrees)) * 60.0) as i16;
        let seconds =
            (self.longitude.abs() - f32::from(degrees) - f32::from(minutes) / 60.0) * 3600.0;
        (degrees, minutes, seconds)
    }

    /// Set the latitude from DMS (degrees, minutes, seconds) format.
    pub fn set_latitude_dms(&mut self, degrees: i16, minutes: i16, seconds: f32) {
        self.latitude = f32::from(degrees) + f32::from(minutes) / 60.0 + seconds / 3600.0;
    }

    /// Set the longitude from DMS (degrees, minutes, seconds) format.
    pub fn set_longitude_dms(&mut self, degrees: i16, minutes: i16, seconds: f32) {
        self.longitude = f32::from(degrees) + f32::from(minutes) / 60.0 + seconds / 3600.0;
    }

    /// Calculate the distance in kilometers between two GPS coordinates
    pub fn distance_to(&self, other: &Self) -> f32 {
        // This is a simple implementation of the Haversine formula.
        // It is not the most accurate, but it is good enough for our purposes.
        let earth_radius_km = 6371.0; // Earth's radius in kilometers
        let delta_latitude = (other.latitude - self.latitude).to_radians();
        let delta_longitude = (other.longitude - self.longitude).to_radians();
        let a = (delta_latitude / 2.0).sin().mul_add(
            (delta_latitude / 2.0).sin(),
            self.latitude.to_radians().cos()
                * other.latitude.to_radians().cos()
                * (delta_longitude / 2.0).sin()
                * (delta_longitude / 2.0).sin(),
        );
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        earth_radius_km * c
    }

    /// Calculate the bearing from this GPS coordinate to another. The bearing is expressed in degrees clockwise from true north.
    pub fn bearing_to(&self, other: &Self) -> f32 {
        let delta_longitude = (other.longitude - self.longitude).to_radians();
        let y = delta_longitude.sin() * other.latitude.to_radians().cos();
        let x = self.latitude.to_radians().cos().mul_add(
            other.latitude.to_radians().sin(),
            -(self.latitude.to_radians().sin()
                * other.latitude.to_radians().cos()
                * delta_longitude.cos()),
        );
        let bearing_degrees = y.atan2(x).to_degrees();
        (bearing_degrees + 360.0) % 360.0 // Make sure we're in the range 0..360
    }

    /// Calculate the midpoint between this GPS coordinate and another.
    pub fn midpoint(&self, other: &Self) -> Self {
        let delta_longitude = (other.longitude - self.longitude).to_radians();
        let latitude1 = self.latitude.to_radians();
        let latitude2 = other.latitude.to_radians();
        let longitude1 = self.longitude.to_radians();
        let bx = (other.latitude - self.latitude).to_radians().cos()
            * delta_longitude.cos()
            * latitude2.cos();
        let by = (other.latitude - self.latitude).to_radians().cos()
            * delta_longitude.cos()
            * latitude2.sin();
        let latitude = (latitude1 + latitude2).atan2(
            (self.latitude.to_radians().sin() + other.latitude.to_radians().sin()).hypot(bx),
        );
        let longitude = longitude1 + delta_longitude.cos().atan2(by);
        Self::new(latitude.to_degrees(), longitude.to_degrees())
    }

    /// Calculate the destination point given a bearing and distance in kilometers. The bearing is expressed in degrees clockwise from true north.
    pub fn destination(&self, bearing_degrees: f32, distance_km: f32) -> Self {
        let earth_radius_km = 6371.0; // Earth's radius in kilometers
        let angular_distance = distance_km / earth_radius_km;
        let bearing = bearing_degrees.to_radians();
        let latitude = self.latitude.to_radians();
        let longitude = self.longitude.to_radians();
        let latitude2 = latitude.sin().mul_add(
            angular_distance.cos(),
            latitude.cos() * angular_distance.sin() * bearing.cos(),
        );
        let longitude2 = longitude
            + (bearing.sin() * angular_distance.sin() * latitude.cos()).atan2(
                latitude
                    .sin()
                    .mul_add(-latitude2.sin(), angular_distance.cos()),
            );
        Self::new(latitude2.to_degrees(), longitude2.to_degrees())
    }
}

impl Display for GpsCoordinate {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.format {
            GpsUnits::Decimal => write!(f, "({}, {})", self.latitude, self.longitude),
            GpsUnits::DMS => {
                let (lat_deg, lat_min, lat_sec) = self.latitude_dms();
                let (lon_deg, lon_min, lon_sec) = self.longitude_dms();

                let lat_deg_abs = lat_deg.abs();
                let lon_deg_abs = lon_deg.abs();

                // Check if the latitude is positive or negative and format accordingly.
                let lat_dir = if self.latitude >= 0.0 { 'N' } else { 'S' };
                let lon_dir = if self.longitude >= 0.0 { 'E' } else { 'W' };

                write!(
                    f,
                    "({lat_deg_abs}°{lat_min}'{lat_sec}\"{lat_dir}, {lon_deg_abs}°{lon_min}'{lon_sec}\"{lon_dir})"
                )
            }
        }
    }
}

impl Default for GpsCoordinate {
    /// Default to Null Island. (0.0, 0.0)
    ///
    /// Null Island is a fictional island located at 0°N, 0°E. <https://en.wikipedia.org/wiki/Null_Island>
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gps_coordinate() {
        let coord = GpsCoordinate::new(37.7749, -122.4194);
        assert_eq!(coord.latitude_decimal(), 37.7749);
        assert_eq!(coord.longitude_decimal(), -122.4194);
        assert_eq!(coord.latitude_dms(), (37, 46, 29.634_762));
        assert_eq!(coord.longitude_dms(), (122, 25, 9.85111));

        let mut coord = GpsCoordinate::default();
        coord.set_latitude_dms(37, 46, 29.64);
        coord.set_longitude_dms(122, 25, 10.0);
        assert_eq!(coord.latitude_decimal(), 37.7749);
        assert_eq!(coord.longitude_decimal(), 122.41944);

        coord.set_latitude_decimal(37.7749);
        coord.set_longitude_decimal(-122.4194);
        assert_eq!(coord.latitude_dms(), (37, 46, 29.634_762));
        assert_eq!(coord.longitude_dms(), (122, 25, 9.85111));

        let coord1 = GpsCoordinate::new(37.7749, -122.4194);
        let coord2 = GpsCoordinate::new(34.0522, -118.2437);
        assert_eq!(coord1.distance_to(&coord2), 559.12067);
        assert_eq!(coord1.bearing_to(&coord2), 136.50287);

        let midpoint = coord1.midpoint(&coord2);
        assert_eq!(midpoint.latitude_decimal(), 41.171_646);
        assert_eq!(midpoint.longitude_decimal(), -61.614_407);

        let destination = coord1.destination(118.0, 559.0);
        assert_eq!(destination.latitude_decimal(), 33.099_026);
        assert_eq!(destination.longitude_decimal(), -117.13848);

        let titanic_wreck = GpsCoordinate::new(41.7325, -49.9469);
        assert_eq!(titanic_wreck.latitude_dms(), (41, 43, 56.993_507));
        assert_eq!(titanic_wreck.longitude_dms(), (49, 56, 48.837_875));

        // Test the formatting
        let mut coord = GpsCoordinate::new(37.7749, -122.4194);
        assert_eq!(format!("{coord}"), "(37.7749, -122.4194)");

        assert_eq!(coord.format(), GpsUnits::Decimal);
        coord.set_format(GpsUnits::DMS);
        assert_eq!(
            format!("{coord}"),
            "(37°46'29.634762\"N, 122°25'9.85111\"W)"
        );

        assert_eq!(coord.format(), GpsUnits::DMS);
    }
}
