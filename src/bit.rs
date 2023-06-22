pub struct Bit {
    pub value: bool,
}

impl Bit {
    pub fn to_string(&self) -> String {
        match self.value {
            true => String::from("1"),
            false => String::from("0"),
        }
    }

    pub fn add(&self, other: &Bit) -> (Bit, Bit) {
        match (self.value, other.value) {
            (false, false) => (Bit { value: false }, Bit { value: false }),
            (false, true) => (Bit { value: false }, Bit { value: true }),
            (true, false) => (Bit { value: false }, Bit { value: true }),
            (true, true) => (Bit { value: true }, Bit { value: true }),
        }
    }
}
