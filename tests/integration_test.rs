use ucon::{Length, Temperature, Time, Volume, Weight};

#[test]
fn test_length_conversion() {
    let meter = Length::Meter(1.0);
    let kilometer = Length::Kilometer(0.001);

    assert_eq!(
        meter.convert_to(&Length::Kilometer(0.0)).unwrap(),
        kilometer
    );
}

#[test]
fn test_weight_conversion() {
    let kilogram = Weight::Kilogram(1.0);
    let gram = Weight::Gram(1000.0);

    assert_eq!(kilogram.convert_to(&Weight::Gram(0.0)).unwrap(), gram);
}

#[test]
fn test_temperature_conversion() {
    let celsius = Temperature::Celsius(25.0);
    let fahrenheit = Temperature::Fahrenheit(77.0);

    assert_eq!(
        celsius.convert_to(&Temperature::Fahrenheit(0.0)).unwrap(),
        fahrenheit
    );
}

#[test]
fn test_volume_conversion() {
    let liter = Volume::Liter(1.0);
    let milliliter = Volume::Milliliter(1000.0);

    assert_eq!(
        liter.convert_to(&Volume::Milliliter(0.0)).unwrap(),
        milliliter
    );
}

#[test]
fn test_time_conversion() {
    let second = Time::Second(3600.0);
    let hour = Time::Hour(1.0);

    assert_eq!(second.convert_to(&Time::Hour(0.0)).unwrap(), hour);
}
