#[cfg(test)]
mod tests {
    use leetcode::single_number::*;

    #[test]
    fn test_single_number_case1() {
        let nums = vec![2, 2, 1];
        let want = 1;

        let result = single_number(nums);

        assert_eq!(result, want);
    }

    #[test]
    fn test_single_number_case2() {
        let nums = vec![4, 1, 2, 1, 2];
        let want = 4;

        let result = single_number(nums);

        assert_eq!(result, want);
    }

    #[test]
    fn test_single_number_case3() {
        let nums = vec![1];
        let want = 1;

        let result = single_number(nums);

        assert_eq!(result, want);
    }
}
