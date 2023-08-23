pub struct Solution;

pub trait FindIndexOfTheFirstOccurInAString {
    fn str_str(haystack: String, needle: String) -> i32;
}

impl FindIndexOfTheFirstOccurInAString for Solution {
    fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_length = haystack.len();
        let needle_length = needle.len();
        if haystack_length < needle_length {
            return -1;
        } else {
            for i in 0..=haystack_length - needle_length {
                if haystack[i..i + needle_length] == needle {
                    return i as i32;
                }
            }
        }
        -1
    }
}
