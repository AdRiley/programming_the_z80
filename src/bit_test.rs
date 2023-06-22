#[cfg(test)]
mod tests {
    use crate::bit::Bit;

    ///////////// to_string tests ///////////////////
    #[test]
    fn test_true_bit_prints_as_1() {
        let true_bit = Bit { value: true };
        let result = true_bit.to_string();
        assert_eq!(result, "1");
    }
    #[test]
    fn test_false_bit_prints_as_0() {
        let false_bit = Bit { value: false };
        let result = false_bit.to_string();
        assert_eq!(result, "0");
    }
    ///////////// add tests ///////////////////
    #[test]
    fn test_zero_bit_plus_zero_bit_is_zero() {
        let false_bit1 = Bit { value: false };
        let false_bit2 = Bit { value: false };
        let (carry, result) = false_bit1.add(&false_bit2);
        assert_eq!(result.to_string(), "0");
        assert_eq!(carry.to_string(), "0");
    }
    #[test]
    fn test_zero_bit_plus_one_bit_is_one() {
        let false_bit1 = Bit { value: false };
        let false_bit2 = Bit { value: true };
        let (carry, result) = false_bit1.add(&false_bit2);
        assert_eq!(result.to_string(), "1");
        assert_eq!(carry.to_string(), "0");
    }
    #[test]
    fn test_one_bit_plus_zero_bit_is_one() {
        let false_bit1 = Bit { value: true };
        let false_bit2 = Bit { value: false };
        let (carry, result) = false_bit1.add(&false_bit2);
        assert_eq!(result.to_string(), "1");
        assert_eq!(carry.to_string(), "0");
    }
    #[test]
    fn test_one_bit_plus_one_bit_is_one_carry_one() {
        let false_bit1 = Bit { value: true };
        let false_bit2 = Bit { value: true };
        let (carry, result) = false_bit1.add(&false_bit2);
        assert_eq!(result.to_string(), "1");
        assert_eq!(carry.to_string(), "1");
    }
}
