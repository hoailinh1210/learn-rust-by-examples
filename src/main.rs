fn main() {
    println!("Hello, world!");

    print_fizzbuzz_to(10);
}

fn is_divisible(n: u32, divisor: u32) -> bool {
    if divisor == 0 {
        false;
    }

    n % divisor == 0
}

fn fizzbuzz(n: u32) -> String {
    let fizz: &str = if is_divisible(n, 3) { "fizz" } else { "" };
    let buzz: &str = if is_divisible(n, 5) { "buzz" } else { "" };

    if fizz.is_empty() && buzz.is_empty() {
        return format!("{n}");
    }

    format!("{fizz}{buzz}")
}

fn print_fizzbuzz_to(n: u32) {
   for i in 1..n {
        println!("{}", fizzbuzz(i));
   } 
}