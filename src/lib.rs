pub mod measurements {
    pub mod area;
    pub mod data;
    pub mod length;
    pub mod speed;
    pub mod temperature;
    pub mod time;
    pub mod volume;
    pub mod weight;
}

pub use measurements::area::Area;
pub use measurements::data::Data;
pub use measurements::length::Length;
pub use measurements::speed::Speed;
pub use measurements::temperature::Temperature;
pub use measurements::time::Time;
pub use measurements::volume::Volume;
pub use measurements::weight::Weight;

#[derive(Debug)]
pub enum ConversionError {
    InvalidUnit,
    ConversionNotPossible,
}
