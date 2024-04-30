#[cfg(test)]
mod tests {
    use leetcode::summary_ranges::*;

    #[test]
    fn test_summary_ranges_case1() {
        let nums = vec![0, 1, 2, 4, 5, 7];
        let want = vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()];

        let result = summary_ranges(nums);

        assert_eq!(result, want);
    }

    #[test]
    fn test_summary_ranges_case2() {
        let nums = vec![0, 2, 3, 4, 6, 8, 9];
        let want = vec![
            "0".to_string(),
            "2->4".to_string(),
            "6".to_string(),
            "8->9".to_string(),
        ];

        let result = summary_ranges(nums);

        assert_eq!(result, want);
    }
}
