use crate::bit::Bit;

pub mod bit;
mod bit_test;

fn main() {
    let bit1 = Bit::from(1);
    let p = bit1.to_string();
    println!("Hello, world! {p}");
}
