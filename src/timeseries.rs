//! Time Series definitions
//!

use chrono::prelude::{UTC, DateTime};
use chrono::TimeZone;


#[derive(Debug, PartialEq)]
pub struct DataPoint {
    index: DateTime<UTC>,
    value: f64
}

#[derive(Debug, PartialEq)]
pub struct TimeSeries {
    data: Vec<DataPoint>,
}


impl DataPoint {
    pub fn new(idx: DateTime<UTC>, val: f64) -> DataPoint {
        DataPoint { index: idx, value: val }
    }
}


impl TimeSeries {
    /// Create Time Series from DataPoints
    pub fn new(data: Vec<DataPoint>) -> TimeSeries {
        TimeSeries { data: data }
    }

    /// Create TS from values.Time index will be created from timestamp
    pub fn from_values(data: Vec<f64>) -> TimeSeries {
        let xs = data.iter().enumerate()
            .map(|(i, x)| DataPoint::new(UTC.timestamp(i as i64, 0), x.clone()))
            .collect();
        TimeSeries { data:  xs}
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Get list of values
    pub fn values(&self) -> Vec<f64> {
        self.data.iter().map(|dp| dp.value).collect()
    }
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::{UTC, TimeZone};

    #[test]
    fn test_new() {
        let a = TimeSeries::new(vec![ DataPoint::new(UTC.ymd(2014, 1, 23).and_hms(9, 10, 11), 4.5),
                                      DataPoint::new(UTC.ymd(2014, 1, 24).and_hms(9, 10, 11), 3.0)]);
        assert!(a.len() == 2);
    }

    #[test]
    fn test_values() {
        let a = TimeSeries::from_values(vec![4.5, 3.5, 8.9]);
        assert!(a.values() == vec![4.5, 3.5, 8.9]);
    }
}