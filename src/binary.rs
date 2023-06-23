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
}
