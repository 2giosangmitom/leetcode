#[cfg(test)]
mod tests {
    use leetcode::reverse_integer::*;

    #[test]
    fn test_reverse_integer_case1() {
        let x = 123;
        let want = 321;

        let result = reverse(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_reverse_integer_case2() {
        let x = -123;
        let want = -321;

        let result = reverse(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_reverse_integer_case3() {
        let x = 120;
        let want = 21;

        let result = reverse(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_reverse_integer_case4() {
        let x = 1534236469;
        let want = 0;

        let result = reverse(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_reverse_integer_case5() {
        let x = -2147483648;
        let want = 0;

        let result = reverse(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_reverse_integer_case6() {
        let x = 2147412349;
        let want = 0;

        let result = reverse(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_reverse_integer_case7() {
        let x = 900000;
        let want = 9;

        let result = reverse(x);

        assert_eq!(result, want);
    }
}
