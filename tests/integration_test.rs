use ucon::{Length, Weight};

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
fn test_invalid_conversion() {
    let meter = Length::Meter(1.0);
    let kilogram = Weight::Kilogram(1.0);

    assert_eq!(
        meter.convert_to(&Length::Kilometer(0.0)).unwrap(),
        Length::Kilometer(0.001)
    );
    assert!(meter.convert_to(&Length::Foot(0.0)).is_ok());
    assert!(kilogram.convert_to(&Weight::Pound(0.0)).is_ok());
}
