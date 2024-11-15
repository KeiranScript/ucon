use crate::ConversionError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Data {
    Byte(f64),
    Kilobyte(f64),
    Megabyte(f64),
    Gigabyte(f64),
    Terabyte(f64),
}

impl Data {
    pub fn convert_to(&self, target_unit: &Data) -> Result<Data, ConversionError> {
        match self {
            Data::Byte(value) => match target_unit {
                Data::Kilobyte(_) => Ok(Data::Kilobyte(value / 1024.0)),
                Data::Megabyte(_) => Ok(Data::Megabyte(value / 1_048_576.0)),
                Data::Gigabyte(_) => Ok(Data::Gigabyte(value / 1_073_741_824.0)),
                Data::Terabyte(_) => Ok(Data::Terabyte(value / 1_099_511_627_776.0)),
                _ => Err(ConversionError::ConversionNotPossible),
            },
            _ => Err(ConversionError::ConversionNotPossible),
        }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Data::Byte(value) => write!(f, "{} B", value),
            Data::Kilobyte(value) => write!(f, "{} KB", value),
            Data::Megabyte(value) => write!(f, "{} MB", value),
            Data::Gigabyte(value) => write!(f, "{} GB", value),
            Data::Terabyte(value) => write!(f, "{} TB", value),
        }
    }
}
