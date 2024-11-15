use crate::ConversionError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Volume {
    Liter(f64),
    Milliliter(f64),
    CubicMeter(f64),
    CubicCentimeter(f64),
}

impl Volume {
    pub fn convert_to(&self, target_unit: &Volume) -> Result<Volume, ConversionError> {
        match self {
            Volume::Liter(value) => match target_unit {
                Volume::Milliliter(_) => Ok(Volume::Milliliter(value * 1000.0)),
                Volume::CubicMeter(_) => Ok(Volume::CubicMeter(value / 1000.0)),
                Volume::CubicCentimeter(_) => Ok(Volume::CubicCentimeter(value * 1000.0)),
                _ => Err(ConversionError::ConversionNotPossible),
            },
            _ => Err(ConversionError::ConversionNotPossible),
        }
    }
}

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Volume::Liter(value) => write!(f, "{} L", value),
            Volume::Milliliter(value) => write!(f, "{} mL", value),
            Volume::CubicMeter(value) => write!(f, "{} m³", value),
            Volume::CubicCentimeter(value) => write!(f, "{} cm³", value),
        }
    }
}
