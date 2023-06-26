use crate::bit::Bit;

pub struct Binary {
    pub value: Vec<Bit>,
}

impl Binary {
    pub fn from(value: &str) -> Binary {
        let mut result = Vec::new();
        for c in value.chars().rev() {
            result.push(Bit::from(c));
        }
        Binary { value: result }
    }
    pub fn from_decimal(value: u128) -> Binary {
        let mut result = Vec::new();
        let mut number = value;
        while number != 0 {
            result.push(match number % 2 {
                1 => Bit::One,
                _ => Bit::Zero,
            });
            number /= 2;
        }
        Binary { value: result }
    }
    pub fn to_decimal(&self) -> u128 {
        let mut result = 0;
        let mut current_power_of_two = 1;
        for b in self.value.iter() {
            if b == &Bit::One {
                result += current_power_of_two
            }
            current_power_of_two *= 2;
        }
        result
    }
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for b in self.value.iter().rev() {
            result += &b.to_string();
        }
        result
    }
    pub fn add(&self, other: Binary) -> Binary {
        let mut result = Vec::new();
        let mut carry = Bit::Zero;
        let mut answer: Bit;
        for it in self.value.iter().zip(other.value.iter()) {
            let (bit1, bit2) = it;
            (carry, answer) = add_three_bits(bit1, bit2, &carry);
            result.push(answer);
        }
        if self.value.len() > other.value.len() {
            for i in other.value.len()..self.value.len() {
                (carry, answer) = add_three_bits(&Bit::Zero, &self.value[i], &carry);
                result.push(answer);
            }
        } else if self.value.len() < other.value.len() {
            for i in self.value.len()..other.value.len() {
                (carry, answer) = add_three_bits(&Bit::Zero, &other.value[i], &carry);
                result.push(answer);
            }
        }
        if carry == Bit::One {
            result.push(carry);
        }
        Binary { value: result }
    }
}

fn add_three_bits(bit1: &Bit, bit2: &Bit, bit3: &Bit) -> (Bit, Bit) {
    match (bit1, bit2, bit3) {
        (Bit::Zero, Bit::Zero, Bit::Zero) => (Bit::Zero, Bit::Zero),
        (Bit::Zero, Bit::Zero, Bit::One) => (Bit::Zero, Bit::One),
        (Bit::Zero, Bit::One, Bit::Zero) => (Bit::Zero, Bit::One),
        (Bit::One, Bit::Zero, Bit::Zero) => (Bit::Zero, Bit::One),
        (Bit::Zero, Bit::One, Bit::One) => (Bit::One, Bit::Zero),
        (Bit::One, Bit::One, Bit::Zero) => (Bit::One, Bit::Zero),
        (Bit::One, Bit::Zero, Bit::One) => (Bit::One, Bit::Zero),
        (Bit::One, Bit::One, Bit::One) => (Bit::One, Bit::One),
    }
}
