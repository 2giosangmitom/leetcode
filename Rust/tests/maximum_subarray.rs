#[cfg(test)]
mod tests {
    use leetcode::maximum_subarray::*;

    #[test]
    fn test_maximum_subarray_case1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let want = 6;
        let result = max_sub_array(nums);
        assert_eq!(result, want);
    }

    #[test]
    fn test_maximum_subarray_case2() {
        let nums = vec![0];
        let want = 0;
        let result = max_sub_array(nums);
        assert_eq!(result, want);
    }

    #[test]
    fn test_maximum_subarray_case3() {
        let nums = vec![5, 4, -1, 7, 8];
        let want = 23;
        let result = max_sub_array(nums);
        assert_eq!(result, want);
    }
}
