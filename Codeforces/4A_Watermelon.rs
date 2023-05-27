// 4A - Watermelon
use std::io;

fn main() {
    let mut weight = String::new();
    io::stdin()
        .read_line(&mut weight)
        .expect("Didn't receive input");
    let weight: u32 = weight
        .trim()
        .parse()
        .expect("weight entered wasn't a number");
    if weight % 2 == 0 && weight != 2 {
        print!("YES");
    } else {
        print!("NO");
    }
}
