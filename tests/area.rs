mod tests {
    use ucon::measurements::area::Area;

    #[test]
    fn test_area_conversion() {
        let square_meter = Area::SquareMeter(1.0);
        let square_kilometer = square_meter
            .convert_to(&Area::SquareKilometer(0.0))
            .unwrap();
        assert_eq!(square_kilometer, Area::SquareKilometer(0.000001));
    }
}
