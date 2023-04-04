// https://codelearn.io/learning/thuat-toan-can-ban/3831
use std::io;
use std::io::Write;

fn is_prime(n: u32) -> bool {
    let mut result = true;
    if n < 2 {
        result = false;
    }
    if n % 2 == 0 {
        result = false;
    }
    if n == 2 {
        result = true;
    }
    for i in (3..(n + 1) / 2).step_by(2) {
        if n % i == 0 {
            result = false;
        }
    }
    result
}

fn main() {
    let mut n = String::new();
    print!("n = ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n).expect("Failed to read line!");
    let n: u32 = n.trim().parse().expect("n entered wasn't a number!");
    if is_prime(n) == true {
        println!("True");
    } else {
        println!("False");
    };
}
