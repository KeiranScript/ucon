use std::fmt;

#[derive(Debug, PartialEq)]
pub enum DataStorage {
    Byte(f64),
    Kilobyte(f64),
    Megabyte(f64),
    Gigabyte(f64),
    Terabyte(f64),
}

impl DataStorage {
    pub fn convert_to(
        &self,
        target_unit: &DataStorage,
    ) -> Result<DataStorage, super::ConversionError> {
        match self {
            DataStorage::Byte(value) => match target_unit {
                DataStorage::Kilobyte(_) => Ok(DataStorage::Kilobyte(value / 1024.0)),
                DataStorage::Megabyte(_) => Ok(DataStorage::Megabyte(value / 1_048_576.0)),
                DataStorage::Gigabyte(_) => Ok(DataStorage::Gigabyte(value / 1_073_741_824.0)),
                DataStorage::Terabyte(_) => Ok(DataStorage::Terabyte(value / 1_099_511_627_776.0)),
                _ => Err(super::ConversionError::ConversionNotPossible),
            },
            _ => Err(super::ConversionError::ConversionNotPossible),
        }
    }
}

impl fmt::Display for DataStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataStorage::Byte(value) => write!(f, "{} B", value),
            DataStorage::Kilobyte(value) => write!(f, "{} KB", value),
            DataStorage::Megabyte(value) => write!(f, "{} MB", value),
            DataStorage::Gigabyte(value) => write!(f, "{} GB", value),
            DataStorage::Terabyte(value) => write!(f, "{} TB", value),
        }
    }
}
