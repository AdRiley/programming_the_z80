#[cfg(test)]
mod tests {
    use crate::bit::Bit;
    ///////////// from tests ///////////////////
    #[test]
    fn test_from_0_gives_zero_bit() {
        let result = Bit::from(0);
        assert_eq!(result.to_string(), "0");
    }
    #[test]
    fn test_from_1_gives_one_bit() {
        let result = Bit::from(1);
        assert_eq!(result.to_string(), "1");
    }
    #[test]
    #[should_panic]
    fn test_from_out_of_range_panics() {
        let _result = Bit::from(42);
    }
    ///////////// to_string tests ///////////////////
    #[test]
    fn test_1_bit_prints_as_1() {
        let true_bit = Bit::from(1);
        let result = true_bit.to_string();
        assert_eq!(result, "1");
    }
    #[test]
    fn test_0_bit_prints_as_0() {
        let false_bit = Bit::from(0);
        let result = false_bit.to_string();
        assert_eq!(result, "0");
    }
    ///////////// add tests ///////////////////
    #[test]
    fn test_zero_bit_plus_zero_bit_is_zero() {
        let false_bit1 = Bit::from(0);
        let false_bit2 = Bit::from(0);
        let (carry, result) = false_bit1.add(&false_bit2);
        assert_eq!(result.to_string(), "0");
        assert_eq!(carry.to_string(), "0");
    }
    #[test]
    fn test_zero_bit_plus_one_bit_is_one() {
        let false_bit1 = Bit::from(0);
        let false_bit2 = Bit::from(1);
        let (carry, result) = false_bit1.add(&false_bit2);
        assert_eq!(result.to_string(), "1");
        assert_eq!(carry.to_string(), "0");
    }
    #[test]
    fn test_one_bit_plus_zero_bit_is_one() {
        let false_bit1 = Bit::from(1);
        let false_bit2 = Bit::from(0);
        let (carry, result) = false_bit1.add(&false_bit2);
        assert_eq!(result.to_string(), "1");
        assert_eq!(carry.to_string(), "0");
    }
    #[test]
    fn test_one_bit_plus_one_bit_is_one_carry_one() {
        let false_bit1 = Bit::from(1);
        let false_bit2 = Bit::from(1);
        let (carry, result) = false_bit1.add(&false_bit2);
        assert_eq!(result.to_string(), "1");
        assert_eq!(carry.to_string(), "1");
    }
}
