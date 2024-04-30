#[cfg(test)]
mod tests {
    use leetcode::plus_one::plus_one;

    #[test]
    fn test_plus_one_case1() {
        let digits = vec![1, 2, 3];
        let want = vec![1, 2, 4];

        let result = plus_one(digits);

        assert_eq!(result, want);
    }

    #[test]
    fn test_plus_one_case2() {
        let digits = vec![4, 3, 2, 1];
        let want = vec![4, 3, 2, 2];

        let result = plus_one(digits);

        assert_eq!(result, want);
    }

    #[test]
    fn test_plus_one_case3() {
        let digits = vec![9];
        let want = vec![1, 0];

        let result = plus_one(digits);

        assert_eq!(result, want);
    }
}
