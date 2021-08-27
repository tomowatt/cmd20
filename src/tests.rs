mod tests {
    use cmd20::*;

    #[test]
    fn test_pad_zero_d0() {
        let result = pad(0, 0);
        assert_eq!("0", result);
    }

    #[test]
    fn test_pad_ten_d0() {
        let result = pad(10, 0);
        assert_eq!("10", result);
    }

    #[test]
    fn test_pad_1000_d0() {
        let result = pad(1000, 0);
        assert_eq!("1000", result);
    }

    #[test]
    fn test_pad_0_d20() {
        let result = pad(0, 20);
        assert_eq!(" 0 ", result);
    }

    #[test]
    fn test_pad_1_d20() {
        let result = pad(1, 20);
        assert_eq!(" 1 ", result);
    }

    #[test]
    fn test_pad_10_d100() {
        let result = pad(10, 20);
        assert_eq!("1 0", result);
    }

    #[test]
    fn test_pad_20_d20() {
        let result = pad(20, 20);
        assert_eq!("2 0", result);
    }

    #[test]
    fn test_pad_1000_d20() {
        let result = pad(1000, 20);
        assert_eq!("1000", result);
    }

    #[test]
    fn test_pad_0_d100() {
        let result = pad(0, 100);
        assert_eq!(" 0 ", result);
    }

    #[test]
    fn test_pad_1_d100() {
        let result = pad(1, 100);
        assert_eq!(" 1 ", result);
    }

    #[test]
    fn test_pad_50_d100() {
        let result = pad(50, 100);
        assert_eq!("5 0", result);
    }

    #[test]
    fn test_pad_100_d100() {
        let result = pad(100, 100);
        assert_eq!("100", result);
    }

    #[test]
    fn test_pad_1000_d100() {
        let result = pad(1000, 100);
        assert_eq!("1000", result);
    }
}
