mod tests {
    #[test]
    fn test_conversion_error() {
        assert_eq!(
            format!("{:?}", ucon::ConversionError::InvalidUnit),
            "InvalidUnit"
        );
        assert_eq!(
            format!("{:?}", ucon::ConversionError::ConversionNotPossible),
            "ConversionNotPossible"
        );
    }
}
