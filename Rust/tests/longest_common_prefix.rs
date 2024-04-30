#[cfg(test)]
mod tests {
    use leetcode::longest_common_prefix::*;

    #[test]
    fn test_longest_common_prefix_case1() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let want = "fl".to_string();
        let result = longest_common_prefix(strs);
        assert_eq!(result, want);
    }

    #[test]
    fn test_longest_common_prefix_case2() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let want = "".to_string();
        let result = longest_common_prefix(strs);
        assert_eq!(result, want);
    }

    #[test]
    fn test_longest_common_prefix_case3() {
        let strs = vec!["chi".to_string(), "chien".to_string(), "chau".to_string()];
        let want = "ch".to_string();
        let result = longest_common_prefix(strs);
        assert_eq!(result, want);
    }
}
