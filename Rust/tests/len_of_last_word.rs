#[cfg(test)]
mod tests {
    use leetcode::len_of_last_word::length_of_last_word;

    #[test]
    fn test_length_of_last_word_case1() {
        let s = "Hello World".to_string();
        let want = 5;
        let result = length_of_last_word(s);
        assert_eq!(result, want);
    }

    #[test]
    fn test_length_of_last_word_case2() {
        let s = "   fly me   to   the moon  ".to_string();
        let want = 4;
        let result = length_of_last_word(s);
        assert_eq!(result, want);
    }

    #[test]
    fn test_length_of_last_word_case3() {
        let s = "luffy is still joyboy".to_string();
        let want = 6;
        let result = length_of_last_word(s);
        assert_eq!(result, want);
    }
}
