pub fn is_palindrome(x: i32) -> bool {
    let reverse_number = |n: &i32| -> i32 {
        let mut result = 0;
        let mut temp = *n;
        while temp != 0 {
            let last_digit = temp % 10;
            result = result * 10 + last_digit;
            temp /= 10;
        }
        result
    };

    if x < 0 {
        return false;
    }
    let reversed_number = reverse_number(&x);
    x == reversed_number
}
