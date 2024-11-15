mod tests {
    use ucon::measurements::volume::Volume;

    #[test]
    fn test_volume_conversion() {
        let liter = Volume::Liter(1.0);
        let milliliter = liter.convert_to(&Volume::Milliliter(0.0)).unwrap();
        assert_eq!(milliliter, Volume::Milliliter(1000.0));
    }
}
