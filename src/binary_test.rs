#[cfg(test)]
mod tests {
    use crate::{binary::Binary, bit::Bit};

    ///////////// from tests ///////////////////
    #[test]
    fn test_from_string_zero() {
        let result = Binary::from("0");
        assert_eq!(result.value[0], Bit::Zero);
        assert_eq!(result.value.len(), 1);
    }
    #[test]
    fn test_from_string_one() {
        let result = Binary::from("1");
        assert_eq!(result.value[0], Bit::One);
        assert_eq!(result.value.len(), 1);
    }
    #[test]
    fn test_from_string_zero_one() {
        let result = Binary::from("01");
        assert_eq!(result.value[0], Bit::One);
        assert_eq!(result.value[1], Bit::Zero);
        assert_eq!(result.value.len(), 2);
    }
    #[test]
    fn test_from_string_one_one_zero() {
        let result = Binary::from("110");
        assert_eq!(result.value[0], Bit::Zero);
        assert_eq!(result.value[1], Bit::One);
        assert_eq!(result.value[2], Bit::One);
        assert_eq!(result.value.len(), 3);
    }
    #[test]
    #[should_panic]
    fn test_from_non_binary_string_panics() {
        let _result = Binary::from("text");
    }

    ///////////// to_decimal tests ///////////////////
    #[test]
    fn test_to_decimal_0() {
        let result = Binary::from("0").to_decimal();
        assert_eq!(result, 0);
    }
    // p.20 example:1
    #[test]
    fn test_to_decimal_00001001() {
        let result = Binary::from("00001001").to_decimal();
        assert_eq!(result, 9);
    }
    // p.20 example:2
    #[test]
    fn test_to_decimal_10000001() {
        let result = Binary::from("10000001").to_decimal();
        assert_eq!(result, 129);
    }
    // p.21 exercise:1.1
    #[test]
    fn test_to_decimal_11111100() {
        let result = Binary::from("11111100").to_decimal();
        assert_eq!(result, 252);
    }
    #[test]
    fn test_to_decimal_10011() {
        let i = Binary::from("10011");
        let result = i.to_decimal();
        assert_eq!(result, 19);
    }

    ///////////// from_decimal tests ///////////////////
    // p.22 example:1
    #[test]
    fn test_from_decimal_11() {
        let result = Binary::from_decimal(11).to_string();
        assert_eq!(result, "1011");
    }
    // p.22 exercise:1.2
    #[test]
    fn test_from_decimal_257() {
        let result = Binary::from_decimal(257).to_string();
        assert_eq!(result, "100000001");
    }
    // p.22 exercise:1.3
    #[test]
    fn test_from_decimal_19() {
        let result = Binary::from_decimal(19);
        assert_eq!(result.to_string(), "10011");
        assert_eq!(result.to_decimal(), 19);
    }
    ///////////// add tests ///////////////////
    // p.22 example:1
    #[test]
    fn test_add_10_and_11() {
        let binary1 = Binary::from("10");
        let result = binary1.add(Binary::from("01"));
        assert_eq!(result.to_string(), "11");
    }
    // p.23 exercise 1.4
    #[test]
    fn test_add_5_and_10() {
        let binary1 = Binary::from_decimal(5);
        let binary2 = Binary::from_decimal(10);
        let result = binary2.add(binary1);
        assert_eq!(result.to_decimal(), 15);
    }
    // p.23 example:2
    #[test]
    fn test_add_11_and_01() {
        let binary1 = Binary::from("11");
        let result = binary1.add(Binary::from("01"));
        assert_eq!(result.to_string(), "100");
    }
    // p.23 example:3
    #[test]
    fn test_add_111_and_11() {
        let binary1 = Binary::from("111");
        let result = binary1.add(Binary::from("11"));
        assert_eq!(result.to_string(), "1010");
    }
    // p.23 exercise 1.5
    #[test]
    fn test_add_1111_and_1() {
        let binary1 = Binary::from("1111");
        let result = binary1.add(Binary::from("1"));
        assert_eq!(result.to_string(), "10000");
    }
    #[test]
    fn test_add_1_and_1111() {
        let binary1 = Binary::from("1");
        let result = binary1.add(Binary::from("1111"));
        assert_eq!(result.to_string(), "10000");
    }
}
