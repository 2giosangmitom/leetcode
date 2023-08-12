/**
 * Runtime: 0ms (Beats 100%)
 * Memory: 2.2MB (Beats 26.79%)
 */

pub struct Solution;

pub trait ReverseInteger {
    fn reverse(x: i32) -> i32;
}

impl ReverseInteger for Solution {
    fn reverse(mut x: i32) -> i32 {
        let mut result: i32 = 0;
        while x != 0 {
            match result.checked_mul(10) {
                Some(num) => {
                    let last_number = x % 10;
                    if let Some(ok) = num.checked_add(last_number) {
                        result = ok
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
}
