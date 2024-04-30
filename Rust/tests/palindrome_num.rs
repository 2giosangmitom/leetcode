#[cfg(test)]
mod tests {
    use leetcode::palindrome_num::*;

    #[test]
    fn test_palindrome_number_case1() {
        let num = -10;
        let want = false;

        let result = is_palindrome(num);

        assert_eq!(result, want);
    }

    #[test]
    fn test_palindrome_number_case2() {
        let num = 5;
        let want = true;

        let result = is_palindrome(num);

        assert_eq!(result, want);
    }

    #[test]
    fn test_palindrome_number_case3() {
        let num = 121;
        let want = true;

        let result = is_palindrome(num);

        assert_eq!(result, want);
    }

    #[test]
    fn test_palindrome_number_case4() {
        let num = 321;
        let want = false;

        let result = is_palindrome(num);

        assert_eq!(result, want);
    }

    #[test]
    fn test_palindrome_number_case5() {
        let num = 111;
        let want = true;

        let result = is_palindrome(num);

        assert_eq!(result, want);
    }
}
