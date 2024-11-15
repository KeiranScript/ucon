use crate::ConversionError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Weight {
    Kilogram(f64),
    Gram(f64),
    Pound(f64),
    Ounce(f64),
}

impl Weight {
    pub fn convert_to(&self, target_unit: &Weight) -> Result<Weight, ConversionError> {
        match self {
            Weight::Kilogram(value) => match target_unit {
                Weight::Gram(_) => Ok(Weight::Gram(value * 1000.0)),
                Weight::Pound(_) => Ok(Weight::Pound(value * 2.20462)),
                Weight::Ounce(_) => Ok(Weight::Ounce(value * 35.274)),
                _ => Err(ConversionError::ConversionNotPossible),
            },
            _ => Err(ConversionError::ConversionNotPossible),
        }
    }

    pub fn to_stones(&self) -> Result<f64, ConversionError> {
        match self {
            Weight::Kilogram(value) => Ok(value * 0.157473),
            Weight::Gram(value) => Ok(value * 0.000157473),
            Weight::Pound(value) => Ok(value * 0.0714286),
            Weight::Ounce(value) => Ok(value * 0.00446429),
        }
    }
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Weight::Kilogram(value) => write!(f, "{} kg", value),
            Weight::Gram(value) => write!(f, "{} g", value),
            Weight::Pound(value) => write!(f, "{} lb", value),
            Weight::Ounce(value) => write!(f, "{} oz", value),
        }
    }
}
