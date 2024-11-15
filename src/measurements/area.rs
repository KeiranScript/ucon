use crate::ConversionError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Area {
    SquareMeter(f64),
    SquareKilometer(f64),
    SquareMile(f64),
}

impl Area {
    pub fn convert_to(&self, target_unit: &Area) -> Result<Area, ConversionError> {
        match self {
            Area::SquareMeter(value) => match target_unit {
                Area::SquareKilometer(_) => Ok(Area::SquareKilometer(value / 1_000_000.0)),
                Area::SquareMile(_) => Ok(Area::SquareMile(value / 2_589_988.11)),
                _ => Err(ConversionError::ConversionNotPossible),
            },
            _ => Err(ConversionError::ConversionNotPossible),
        }
    }

    pub fn to_square_feet(&self) -> Result<f64, ConversionError> {
        match self {
            Area::SquareMeter(value) => Ok(value * 10.7639),
            Area::SquareKilometer(value) => Ok(value * 1.076e+7),
            Area::SquareMile(value) => Ok(value * 2.788e+7),
        }
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Area::SquareMeter(value) => write!(f, "{} m²", value),
            Area::SquareKilometer(value) => write!(f, "{} km²", value),
            Area::SquareMile(value) => write!(f, "{} mi²", value),
        }
    }
}
