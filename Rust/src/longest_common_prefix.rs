/**
 * Runtime: 0ms (Beats 100%)
 * Memory: 2.2MB (Beats 17.35%)
 */

pub struct Solution;

pub trait LongestCommonPrefix {
    fn longest_common_prefix(strs: Vec<String>) -> String;
}

impl LongestCommonPrefix for Solution {
    fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs[0].clone();
        for i in 1..strs.len() {
            while strs[i].find(prefix.as_str()) != Some(0) {
                prefix.pop();
                if prefix.len() == 0 {
                    return "".to_string();
                }
            }
        }
        prefix
    }
}
