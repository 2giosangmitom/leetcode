#[cfg(test)]
mod tests {
    use leetcode::find_index_of_the1st_occur_in_a_string::str_str;

    #[test]
    fn test_find_index_of_the_first_occur_in_a_string_1() {
        let haystack = "sadbutsad".to_string();
        let needle = "sad".to_string();
        let want = 0;
        assert_eq!(str_str(haystack, needle), want);
    }

    #[test]
    fn test_find_index_of_the_first_occur_in_a_string_2() {
        let haystack = "leetcode".to_string();
        let needle = "leeto".to_string();
        let want = -1;
        assert_eq!(str_str(haystack, needle), want);
    }

    #[test]
    fn test_find_index_of_the_first_occur_in_a_string_3() {
        let haystack = "hello".to_string();
        let needle = "ll".to_string();
        let want = 2;
        assert_eq!(str_str(haystack, needle), want);
    }

    #[test]
    fn test_find_index_of_the_first_occur_in_a_string_4() {
        let haystack = "onepiece".to_string();
        let needle = "pheloiquaaeoi".to_string();
        let want = -1;
        assert_eq!(str_str(haystack, needle), want);
    }
}
