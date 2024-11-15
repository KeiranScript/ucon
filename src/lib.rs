pub mod length;
pub mod temperature;
pub mod time;
pub mod volume;
pub mod weight;

pub use length::Length;
pub use temperature::Temperature;
pub use time::Time;
pub use volume::Volume;
pub use weight::Weight;

#[derive(Debug)]
pub enum ConversionError {
    InvalidUnit,
    ConversionNotPossible,
}
