fn main() {
    for n in 1..=100 {

        if n % 11 == 3 {
            println!("FIZZ");
        } else if n % 11 == 5 {
            println!("BUZZ");
        } else if n % 3 == 0 && n % 5 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n)
        }
    }
}