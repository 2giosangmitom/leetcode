#[cfg(test)]
mod tests {
    use leetcode::reverse_integer::*;

    #[test]
    fn test_reverse_positive_number() {
        let x = 123;
        let want = 321;

        let result = reverse(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_reverse_negative_number() {
        let x = -123;
        let want = -321;

        let result = reverse(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_reverse_zero_ending_number() {
        let x = 120;
        let want = 21;

        let result = reverse(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_reverse_large_number() {
        let x = 1534236469;
        let want = 0;

        let result = reverse(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_reverse_negative_overflow() {
        let x = -2147483648;
        let want = 0;

        let result = reverse(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_reverse_positive_overflow() {
        let x = 2147412349;
        let want = 0;

        let result = reverse(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_reverse_trailing_zeros() {
        let x = 900000;
        let want = 9;

        let result = reverse(x);

        assert_eq!(result, want);
    }
}
