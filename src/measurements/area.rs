use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Area {
    SquareMeter(f64),
    SquareKilometer(f64),
    SquareMile(f64),
}

impl Area {
    pub fn convert_to(&self, target_unit: &Area) -> Result<Area, super::ConversionError> {
        match self {
            Area::SquareMeter(value) => match target_unit {
                Area::SquareKilometer(_) => Ok(Area::SquareKilometer(value / 1_000_000.0)),
                Area::SquareMile(_) => Ok(Area::SquareMile(value / 2_589_988.11)),
                _ => Err(super::ConversionError::ConversionNotPossible),
            },
            _ => Err(super::ConversionError::ConversionNotPossible),
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
