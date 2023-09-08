fn collatz(start: u32) {
	let mut n:u32 = start;

	while n != 1 {
		println!("{}", n);

		if n % 2 == 0 {
			n /= 2;
		} else if n % 2 == 1 {
			n = n * 3 + 1;
		}
	}
	println!("1");
}

fn main() {
	collatz(3);
}