fn min(a: i32, b: i32) -> i32 {
    if a >= b {
        return b;
    } else {
        return a;
    }
}

fn main() {
    println!("return expected: [10]  | actual return: [{}]", min(10, 100));
    println!("return expected: [10]  | actual return: [{}]", min(100, 10));
    println!("return expected: [10]  | actual return: [{}]", min(10, 10));
    println!("return expected: [0]   | actual return: [{}]", min(1, 0));
    println!("return expected: [0]   | actual return: [{}]", min(0, 1));
    println!("return expected: [0]   | actual return: [{}]", min(0, 0));
    println!("return expected: [-10] | actual return: [{}]", min(-10, 0));
    println!("return expected: [-10] | actual return: [{}]", min(0, -10));
    println!("return expected: [-10] | actual return: [{}]", min(-10, -10));
}