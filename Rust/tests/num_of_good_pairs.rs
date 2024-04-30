#[cfg(test)]
mod tests {
    use leetcode::num_of_good_pairs::*;

    #[test]
    fn test_number_of_good_pairs_case1() {
        let nums = vec![1, 2, 3, 1, 1, 3];
        let want = 4;

        let result = num_identical_pairs(nums);

        assert_eq!(result, want);
    }

    #[test]
    fn test_number_of_good_pairs_case2() {
        let nums = vec![1, 1, 1, 1];
        let want = 6;

        let result = num_identical_pairs(nums);

        assert_eq!(result, want);
    }

    #[test]
    fn test_number_of_good_pairs_case3() {
        let nums = vec![1, 2, 3];
        let want = 0;

        let result = num_identical_pairs(nums);

        assert_eq!(result, want);
    }
}
