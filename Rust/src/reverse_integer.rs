pub fn reverse(mut x: i32) -> i32 {
    let mut result: i32 = 0;
    while x != 0 {
        match result.checked_mul(10) {
            Some(num) => {
                let last_digit = x % 10;
                if let Some(ok) = num.checked_add(last_digit) {
                    result = ok;
                } else {
                    return 0;
                }
            }
            None => return 0,
        }
        x /= 10;
    }
    result
}
