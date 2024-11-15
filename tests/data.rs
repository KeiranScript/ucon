mod tests {
    use ucon::measurements::data::Data;

    #[test]
    fn test_data_conversion() {
        let byte = Data::Byte(1024.0);
        let kilobyte = byte.convert_to(&Data::Kilobyte(0.0)).unwrap();
        assert_eq!(kilobyte, Data::Kilobyte(1.0));
    }
}
