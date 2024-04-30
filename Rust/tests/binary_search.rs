#[cfg(test)]
mod tests {
    use leetcode::binary_search::*;

    #[test]
    fn test_binary_search_case1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let want = 4;

        let result = search(nums, target);
        assert_eq!(result, want);
    }

    #[test]
    fn test_binary_search_case2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let want = -1;

        let result = search(nums, target);
        assert_eq!(result, want);
    }
}
