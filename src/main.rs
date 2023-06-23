use crate::bit::Bit;

pub mod binary;
mod binary_test;
pub mod bit;
mod bit_test;

fn main() {
    let bit1 = Bit::One;
    let p = bit1.to_string();
    println!("Hello, world! {p}");
}
