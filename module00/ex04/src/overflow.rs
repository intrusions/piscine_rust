fn overflow(a: u8, b: u8) -> u8 {
    return a + b;
}

fn main() {
    println!("255u8 + 1u8 == {}", overflow(255, 1));
}