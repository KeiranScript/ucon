mod tests {
    use ucon::measurements::speed::Speed;

    #[test]
    fn test_speed_conversion() {
        let meter_per_second = Speed::MeterPerSecond(1.0);
        let kilometer_per_hour = meter_per_second
            .convert_to(&Speed::KilometerPerHour(0.0))
            .unwrap();
        assert_eq!(kilometer_per_hour, Speed::KilometerPerHour(3.6));
    }
}
