#[cfg(test)]
mod tests {
    use leetcode::two_sum::*;

    #[test]
    fn test_two_sum_with_duplicate_elements() {
        let nums = vec![3, 3];
        let target = 6;
        let expected = vec![0, 1];

        let result = two_sum(nums.clone(), target);
        let result2 = two_sum2(nums, target);

        assert_eq!(result, expected);
        assert_eq!(result2, expected);
    }

    #[test]
    fn test_two_sum_with_small_input() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let expected = vec![1, 2];

        let result = two_sum(nums.clone(), target);
        let result2 = two_sum2(nums, target);

        assert_eq!(result, expected);
        assert_eq!(result2, expected);
    }

    #[test]
    fn test_two_sum_with_medium_input() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];

        let result = two_sum(nums.clone(), target);
        let result2 = two_sum2(nums, target);

        assert_eq!(result, expected);
        assert_eq!(result2, expected);
    }

    #[test]
    fn test_two_sum_with_large_input() {
        let nums = vec![2, 3, 4, 1, 25, 6, 7, 8, 9, 10];
        let target = 30;
        let expected = vec![-1];

        let result = two_sum(nums.clone(), target);
        let result2 = two_sum2(nums, target);

        assert_eq!(result, expected);
        assert_eq!(result2, expected);
    }
}
