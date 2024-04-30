#[cfg(test)]
mod tests {
    use leetcode::search_insert_position::*;

    #[test]
    fn test_search_insert_pos_case1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let want = 2;

        let result = search_insert(nums, target);

        assert_eq!(result, want);
    }

    #[test]
    fn test_search_insert_pos_case2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let want = 1;

        let result = search_insert(nums, target);

        assert_eq!(result, want);
    }

    #[test]
    fn test_search_insert_pos_case3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let want = 4;

        let result = search_insert(nums, target);

        assert_eq!(result, want);
    }

    #[test]
    fn test_search_insert_pos_case4() {
        let nums = vec![1, 3, 5, 6];
        let target = 0;
        let want = 0;

        let result = search_insert(nums, target);

        assert_eq!(result, want);
    }
}
