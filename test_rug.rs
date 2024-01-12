//# rug = "1.16.0"

use rug::{Complete, Integer};

// Test for possible bug in rug.

fn main() {
    let fib1e10 =
        Integer::fibonacci(10_000_000_000u32).complete() % Integer::from(10_000_000_000u32);
    println!("{}", fib1e10);
}
