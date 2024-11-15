mod tests {
    use ucon::measurements::length::Length;

    #[test]
    fn test_length_conversion() {
        let meter = Length::Meter(1.0);
        let kilometer = meter.convert_to(&Length::Kilometer(0.0)).unwrap();
        assert_eq!(kilometer, Length::Kilometer(0.001));
    }
}
