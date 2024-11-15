use crate::ConversionError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Time {
    Second(f64),
    Minute(f64),
    Hour(f64),
    Day(f64),
}

impl Time {
    pub fn convert_to(&self, target_unit: &Time) -> Result<Time, ConversionError> {
        match self {
            Time::Second(value) => match target_unit {
                Time::Minute(_) => Ok(Time::Minute(value / 60.0)),
                Time::Hour(_) => Ok(Time::Hour(value / 3600.0)),
                Time::Day(_) => Ok(Time::Day(value / 86400.0)),
                _ => Err(ConversionError::ConversionNotPossible),
            },
            _ => Err(ConversionError::ConversionNotPossible),
        }
    }

    pub fn to_milliseconds(&self) -> Result<f64, ConversionError> {
        match self {
            Time::Second(value) => Ok(value * 1000.0),
            Time::Minute(value) => Ok(value * 60_000.0),
            Time::Hour(value) => Ok(value * 3_600_000.0),
            Time::Day(value) => Ok(value * 86_400_000.0),
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Time::Second(value) => write!(f, "{} s", value),
            Time::Minute(value) => write!(f, "{} min", value),
            Time::Hour(value) => write!(f, "{} h", value),
            Time::Day(value) => write!(f, "{} d", value),
        }
    }
}
