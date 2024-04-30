#[cfg(test)]
mod tests {
    use leetcode::missing_number::*;

    #[test]
    fn test_missing_number_case1() {
        let nums = vec![3, 0, 1];
        let want = 2;

        let result = missing_number(nums);

        assert_eq!(result, want);
    }

    #[test]
    fn test_missing_number_case2() {
        let nums = vec![0, 1];
        let want = 2;

        let result = missing_number(nums);

        assert_eq!(result, want);
    }

    #[test]
    fn test_missing_number_case3() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        let want = 8;

        let result = missing_number(nums);

        assert_eq!(result, want);
    }
}
