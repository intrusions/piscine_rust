fn min(a: i32, b: i32) -> i32 {
	if a < b {
		a
	} else {
		b
	}
}

fn main() {
	println!("min : {}", min(12, 42));
	println!("min : {}", min(42, 12));
	println!("min : {}", min(0, -5));
	println!("min : {}", min(-5, 0));
	println!("min : {}", min(0, 0));
}