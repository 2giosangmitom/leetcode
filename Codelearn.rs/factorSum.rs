// https://codelearn.io/learning/thuat-toan-can-ban/3831
use std::io;
use std::io::Write;

fn solve(mut n: u32) -> u32 {
    let mut k = 2;
    let mut sum = 0;
    while n != 1 {
        while n % k == 0 {
            sum += k;
            n /= k;
        }
        k += 1;
    }
    sum
}

fn factor_sum(mut n: u32) -> u32 {
    while n != solve(n) {
        n = solve(n);
    }
    n
}

fn main() {
    let mut n = String::new();
    print!("n = ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n).expect("Failed to read line!");
    let n: u32 = n.trim().parse().expect("n entered wasn't a number!");
    print!("{}", factor_sum(n));
}
