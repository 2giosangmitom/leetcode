#[cfg(test)]
mod tests {
    use leetcode::remove_duplicates_from_sorted_arr::*;

    #[test]
    fn test_remove_duplicates_from_sorted_array_case1() {
        let mut nums = vec![1, 1, 2];
        let want = 2;
        let want_nums = vec![1, 2];

        let result = remove_duplicates(&mut nums);

        assert_eq!(result, want);
        assert_eq!(&nums[..want as usize], &want_nums);
    }

    #[test]
    fn test_remove_duplicates_from_sorted_array_case2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let want = 5;
        let want_nums = vec![0, 1, 2, 3, 4];

        let result = remove_duplicates(&mut nums);

        assert_eq!(result, want);
        assert_eq!(&nums[..want as usize], &want_nums);
    }
}
