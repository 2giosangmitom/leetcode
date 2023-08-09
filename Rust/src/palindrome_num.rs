pub struct Solution;

pub trait PalindromeNumber {
    fn reverse_number(n: &i32) -> i32;

    fn is_palindrome(x: i32) -> bool;
}

impl PalindromeNumber for Solution {
    fn reverse_number(n: &i32) -> i32 {
        let mut num: i32 = 0;
        let mut temp_num: i32 = *n;
        while temp_num != 0 {
            let last_num: i32 = temp_num % 10;
            num = num * 10 + last_num;
            temp_num /= 10;
        }
        num
    }

    fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let reversed_number: i32 = Self::reverse_number(&x);
        x == reversed_number
    }
}
