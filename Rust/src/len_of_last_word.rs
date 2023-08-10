/**
 * Runtime: 0ms (Beats 100%)
 * Memory: 2.1MB (Beats 57.30%)
 */

pub struct Solution;

pub trait LenOfLastWord {
    fn length_of_last_word(s: String) -> i32;
}

impl LenOfLastWord for Solution {
    fn length_of_last_word(s: String) -> i32 {
        let mut count = 0;
        for i in s.trim().chars().rev() {
            if i.is_whitespace() {
                break;
            }
            count += 1;
        }
        count
    }
}
