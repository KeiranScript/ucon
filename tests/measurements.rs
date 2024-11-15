mod measurements {
    use ucon::measurements::{
        area::Area, data::Data, length::Length, speed::Speed, time::Time, volume::Volume,
    };

    const TOLERANCE: f64 = 1e-5;

    #[test]
    fn test_area_conversion() {
        let square_meter = Area::SquareMeter(1.0);
        let square_kilometer = square_meter
            .convert_to(&Area::SquareKilometer(0.0))
            .unwrap();
        assert_eq!(square_kilometer, Area::SquareKilometer(0.000001));
    }

    #[test]
    fn test_area_to_square_feet() {
        let square_meter = Area::SquareMeter(1.0);
        let square_feet = square_meter.to_square_feet().unwrap();
        let expected_square_feet = 10.7639;
        assert!(
            (square_feet - expected_square_feet).abs() < TOLERANCE,
            "Square feet conversion failed: {} != {}",
            square_feet,
            expected_square_feet
        );
    }

    #[test]
    fn test_data_conversion() {
        let byte = Data::Byte(1024.0);
        let kilobyte = byte.convert_to(&Data::Kilobyte(0.0)).unwrap();
        assert_eq!(kilobyte, Data::Kilobyte(1.0));
    }

    #[test]
    fn test_data_to_bits() {
        let byte = Data::Byte(1.0);
        let bits = byte.to_bits().unwrap();
        let expected_bits = 8.0;
        assert!(
            (bits - expected_bits).abs() < TOLERANCE,
            "Bits conversion failed: {} != {}",
            bits,
            expected_bits
        );
    }

    #[test]
    fn test_length_conversion() {
        let meter = Length::Meter(1.0);
        let kilometer = meter.convert_to(&Length::Kilometer(0.0)).unwrap();
        assert_eq!(kilometer, Length::Kilometer(0.001));
    }

    #[test]
    fn test_length_to_miles() {
        let meter = Length::Meter(1609.34);
        let miles = meter.to_miles().unwrap();
        let expected_miles = 1.0;
        assert!(
            (miles - expected_miles).abs() < TOLERANCE,
            "Miles conversion failed: {} != {}",
            miles,
            expected_miles
        );
    }

    #[test]
    fn test_speed_conversion() {
        let meter_per_second = Speed::MeterPerSecond(1.0);
        let kilometer_per_hour = meter_per_second
            .convert_to(&Speed::KilometerPerHour(0.0))
            .unwrap();
        assert_eq!(kilometer_per_hour, Speed::KilometerPerHour(3.6));
    }

    #[test]
    fn test_speed_to_knots() {
        let meter_per_second = Speed::MeterPerSecond(1.0);
        let knots = meter_per_second.to_knots().unwrap();
        let expected_knots = 1.94384;
        assert!(
            (knots - expected_knots).abs() < TOLERANCE,
            "Knots conversion failed: {} != {}",
            knots,
            expected_knots
        );
    }

    #[test]
    fn test_time_conversion() {
        let second = Time::Second(60.0);
        let minute = second.convert_to(&Time::Minute(0.0)).unwrap();
        assert_eq!(minute, Time::Minute(1.0));
    }

    #[test]
    fn test_time_to_milliseconds() {
        let second = Time::Second(1.0);
        let milliseconds = second.to_milliseconds().unwrap();
        let expected_milliseconds = 1000.0;
        assert!(
            (milliseconds - expected_milliseconds).abs() < TOLERANCE,
            "Milliseconds conversion failed: {} != {}",
            milliseconds,
            expected_milliseconds
        );
    }

    #[test]
    fn test_volume_conversion() {
        let liter = Volume::Liter(1.0);
        let milliliter = liter.convert_to(&Volume::Milliliter(0.0)).unwrap();
        assert_eq!(milliliter, Volume::Milliliter(1000.0));
    }

    #[test]
    fn test_volume_to_gallons() {
        let liter = Volume::Liter(1.0);
        let gallons = liter.to_gallons().unwrap();
        let expected_gallons = 0.264172;
        assert!(
            (gallons - expected_gallons).abs() < TOLERANCE,
            "Gallons conversion failed: {} != {}",
            gallons,
            expected_gallons
        );
    }
}
