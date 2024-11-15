mod tests {
    use ucon::measurements::weight::Weight;

    #[test]
    fn test_weight_conversion() {
        let kilogram = Weight::Kilogram(1.0);
        let gram = kilogram.convert_to(&Weight::Gram(0.0)).unwrap();
        assert_eq!(gram, Weight::Gram(1000.0));
    }
}
