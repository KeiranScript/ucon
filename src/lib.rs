pub mod length;
pub mod weight;

pub use length::Length;
pub use weight::Weight;

#[derive(Debug)]
pub enum ConversionError {
    InvalidUnit,
    ConversionNotPossible,
}
