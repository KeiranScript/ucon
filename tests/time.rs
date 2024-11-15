mod tests {
    use ucon::measurements::time::Time;

    #[test]
    fn test_time_conversion() {
        let second = Time::Second(60.0);
        let minute = second.convert_to(&Time::Minute(0.0)).unwrap();
        assert_eq!(minute, Time::Minute(1.0));
    }
}
