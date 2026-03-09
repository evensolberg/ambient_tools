//! Date-range filtering for `WeatherDataPoint` records.

use chrono::{DateTime, NaiveDate, TimeZone, Utc};

use crate::datapoint::data_point::WeatherDataPoint;

/// Filter records to a date range (both bounds inclusive).
///
/// Records whose `dateutc` cannot be parsed warn and pass through.
#[derive(Debug, Default)]
pub struct DateFilter {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

impl DateFilter {
    #[must_use]
    pub const fn new(from: Option<NaiveDate>, to: Option<NaiveDate>) -> Self {
        Self { from, to }
    }

    /// Return `true` if this filter would accept all records (no bounds set).
    #[must_use]
    pub const fn is_passthrough(&self) -> bool {
        self.from.is_none() && self.to.is_none()
    }

    /// Apply the filter, retaining only records within the date range.
    #[must_use]
    pub fn apply(&self, records: Vec<WeatherDataPoint>) -> Vec<WeatherDataPoint> {
        if self.is_passthrough() {
            return records;
        }
        records
            .into_iter()
            .filter(|r| self.record_passes(r))
            .collect()
    }

    fn record_passes(&self, record: &WeatherDataPoint) -> bool {
        let Some(ms) = record.dateutc else {
            log::warn!("Record missing dateutc — passing through");
            return true;
        };
        let date = ms_to_naive_date(ms);
        if let Some(from) = self.from {
            if date < from {
                return false;
            }
        }
        if let Some(to) = self.to {
            if date > to {
                return false;
            }
        }
        true
    }
}

/// Convert milliseconds-since-epoch to a `NaiveDate` (UTC).
#[must_use]
pub fn ms_to_naive_date(ms: u64) -> NaiveDate {
    let secs = (ms / 1000).cast_signed();
    let dt: DateTime<Utc> = Utc.timestamp_opt(secs, 0).single().unwrap_or_default();
    dt.date_naive()
}

/// Convert milliseconds-since-epoch to a UTC `DateTime<Utc>`.
#[must_use]
pub fn ms_to_datetime(ms: u64) -> DateTime<Utc> {
    let secs = (ms / 1000).cast_signed();
    Utc.timestamp_opt(secs, 0).single().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record_with_date(ms: u64) -> WeatherDataPoint {
        WeatherDataPoint {
            dateutc: Some(ms),
            ..Default::default()
        }
    }

    // 2024-05-01 00:00:00 UTC in milliseconds
    const MAY_1: u64 = 1_714_521_600_000;
    // 2024-05-02 00:00:00 UTC
    const MAY_2: u64 = 1_714_608_000_000;
    // 2024-05-03 00:00:00 UTC
    const MAY_3: u64 = 1_714_694_400_000;

    #[test]
    fn passthrough_when_no_bounds() {
        let f = DateFilter::default();
        assert!(f.is_passthrough());
        let records = vec![record_with_date(MAY_1), record_with_date(MAY_2)];
        assert_eq!(f.apply(records).len(), 2);
    }

    #[test]
    fn from_bound_excludes_earlier() {
        let f = DateFilter::new(Some(NaiveDate::from_ymd_opt(2024, 5, 2).unwrap()), None);
        let records = vec![
            record_with_date(MAY_1),
            record_with_date(MAY_2),
            record_with_date(MAY_3),
        ];
        let filtered = f.apply(records);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].dateutc, Some(MAY_2));
    }

    #[test]
    fn to_bound_excludes_later() {
        let f = DateFilter::new(None, Some(NaiveDate::from_ymd_opt(2024, 5, 2).unwrap()));
        let records = vec![
            record_with_date(MAY_1),
            record_with_date(MAY_2),
            record_with_date(MAY_3),
        ];
        let filtered = f.apply(records);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn both_bounds() {
        let f = DateFilter::new(
            Some(NaiveDate::from_ymd_opt(2024, 5, 2).unwrap()),
            Some(NaiveDate::from_ymd_opt(2024, 5, 2).unwrap()),
        );
        let records = vec![
            record_with_date(MAY_1),
            record_with_date(MAY_2),
            record_with_date(MAY_3),
        ];
        let filtered = f.apply(records);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].dateutc, Some(MAY_2));
    }

    #[test]
    fn record_without_date_passes_through() {
        let f = DateFilter::new(Some(NaiveDate::from_ymd_opt(2024, 5, 2).unwrap()), None);
        let records = vec![WeatherDataPoint {
            dateutc: None,
            ..Default::default()
        }];
        assert_eq!(f.apply(records).len(), 1);
    }

    #[test]
    fn ms_to_naive_date_known_value() {
        let date = ms_to_naive_date(MAY_1);
        assert_eq!(date, NaiveDate::from_ymd_opt(2024, 5, 1).unwrap());
    }
}
