use crate::ConversionError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Speed {
    MeterPerSecond(f64),
    KilometerPerHour(f64),
    MilePerHour(f64),
}

impl Speed {
    pub fn convert_to(&self, target_unit: &Speed) -> Result<Speed, ConversionError> {
        match self {
            Speed::MeterPerSecond(value) => match target_unit {
                Speed::KilometerPerHour(_) => Ok(Speed::KilometerPerHour(value * 3.6)),
                Speed::MilePerHour(_) => Ok(Speed::MilePerHour(value * 2.23694)),
                _ => Err(ConversionError::ConversionNotPossible),
            },
            _ => Err(ConversionError::ConversionNotPossible),
        }
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Speed::MeterPerSecond(value) => write!(f, "{} m/s", value),
            Speed::KilometerPerHour(value) => write!(f, "{} km/h", value),
            Speed::MilePerHour(value) => write!(f, "{} mph", value),
        }
    }
}
