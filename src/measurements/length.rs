use crate::ConversionError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Length {
    Meter(f64),
    Kilometer(f64),
    Centimeter(f64),
    Inch(f64),
    Foot(f64),
}

impl Length {
    pub fn convert_to(&self, target_unit: &Length) -> Result<Length, ConversionError> {
        match self {
            Length::Meter(value) => match target_unit {
                Length::Kilometer(_) => Ok(Length::Kilometer(value / 1000.0)),
                Length::Centimeter(_) => Ok(Length::Centimeter(value * 100.0)),
                Length::Inch(_) => Ok(Length::Inch(value * 39.3701)),
                Length::Foot(_) => Ok(Length::Foot(value * 3.28084)),
                _ => Err(ConversionError::ConversionNotPossible),
            },
            _ => Err(ConversionError::ConversionNotPossible),
        }
    }

    pub fn to_miles(&self) -> Result<f64, ConversionError> {
        match self {
            Length::Meter(value) => Ok(value * 0.000621371),
            Length::Kilometer(value) => Ok(value * 0.621371),
            Length::Centimeter(value) => Ok(value * 0.0000062137),
            Length::Inch(value) => Ok(value * 0.0000157828),
            Length::Foot(value) => Ok(value * 0.000189394),
        }
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Length::Meter(value) => write!(f, "{} m", value),
            Length::Kilometer(value) => write!(f, "{} km", value),
            Length::Centimeter(value) => write!(f, "{} cm", value),
            Length::Inch(value) => write!(f, "{} in", value),
            Length::Foot(value) => write!(f, "{} ft", value),
        }
    }
}
