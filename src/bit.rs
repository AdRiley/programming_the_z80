#[derive(Debug, PartialEq)]
pub enum Bit {
    Zero,
    One,
}

impl Bit {
    pub fn from(value: char) -> Bit {
        match value {
            '0' => Bit::Zero,
            '1' => Bit::One,
            _ => panic!("Must take single 0 or 1"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Bit::Zero => String::from("0"),
            Bit::One => String::from("1"),
        }
    }

    pub fn add(&self, other: &Bit) -> (Bit, Bit) {
        match (self, other) {
            (Bit::Zero, Bit::Zero) => (Bit::Zero, Bit::Zero),
            (Bit::Zero, Bit::One) => (Bit::Zero, Bit::One),
            (Bit::One, Bit::Zero) => (Bit::Zero, Bit::One),
            (Bit::One, Bit::One) => (Bit::One, Bit::One),
        }
    }
}
