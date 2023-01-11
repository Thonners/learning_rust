use std::io;
fn main() {
    println!("Enter the n for the nth Fibonacci number:");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n
        .trim()
        .parse()
        .expect("Unable to read integer from input: {n}");

    let nth_fib = get_nth_fibonacci(n);
    println!("Nth Fibonacci number, f, for n = {n} is f({n}) = {nth_fib}")
}

fn get_nth_fibonacci(n: u32) -> u32 {
    let mut num = 1;
    let mut last_num = 0;
    for _ in 2..=n {
        let nth_fib = num + last_num;
        last_num = num;
        num = nth_fib;
    }
    num
}
