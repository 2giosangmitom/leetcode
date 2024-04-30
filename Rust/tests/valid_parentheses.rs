#[cfg(test)]
mod tests {
    use leetcode::valid_parentheses::*;

    #[test]
    fn test_valid_parentheses_case1() {
        let s = "()".to_string();
        let want = true;

        let result = is_valid(s);

        assert_eq!(result, want);
    }

    #[test]
    fn test_valid_parentheses_case2() {
        let s = "()[]{}".to_string();
        let want = true;

        let result = is_valid(s);

        assert_eq!(result, want);
    }

    #[test]
    fn test_valid_parentheses_case3() {
        let s = "(]".to_string();
        let want = false;

        let result = is_valid(s);

        assert_eq!(result, want);
    }

    #[test]
    fn test_valid_parentheses_case4() {
        let s = "){".to_string();
        let want = false;

        let result = is_valid(s);

        assert_eq!(result, want);
    }

    #[test]
    fn test_valid_parentheses_case5() {
        let s = "][]".to_string();
        let want = false;

        let result = is_valid(s);

        assert_eq!(result, want);
    }
}
