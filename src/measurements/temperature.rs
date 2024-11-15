use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

impl Temperature {
    pub fn convert_to(
        &self,
        target_unit: &Temperature,
    ) -> Result<Temperature, super::ConversionError> {
        match self {
            Temperature::Celsius(value) => match target_unit {
                Temperature::Fahrenheit(_) => Ok(Temperature::Fahrenheit(value * 1.8 + 32.0)),
                Temperature::Kelvin(_) => Ok(Temperature::Kelvin(value + 273.15)),
                _ => Err(super::ConversionError::ConversionNotPossible),
            },
            Temperature::Fahrenheit(value) => match target_unit {
                Temperature::Celsius(_) => Ok(Temperature::Celsius((value - 32.0) / 1.8)),
                Temperature::Kelvin(_) => Ok(Temperature::Kelvin((value - 32.0) / 1.8 + 273.15)),
                _ => Err(super::ConversionError::ConversionNotPossible),
            },
            Temperature::Kelvin(value) => match target_unit {
                Temperature::Celsius(_) => Ok(Temperature::Celsius(value - 273.15)),
                Temperature::Fahrenheit(_) => {
                    Ok(Temperature::Fahrenheit((value - 273.15) * 1.8 + 32.0))
                }
                _ => Err(super::ConversionError::ConversionNotPossible),
            },
        }
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Temperature::Celsius(value) => write!(f, "{} °C", value),
            Temperature::Fahrenheit(value) => write!(f, "{} °F", value),
            Temperature::Kelvin(value) => write!(f, "{} K", value),
        }
    }
}
