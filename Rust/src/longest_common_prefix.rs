pub struct Solution;

pub trait LongestCommonPrefix {
    fn longest_common_prefix(strs: Vec<String>) -> String;
}

impl LongestCommonPrefix for Solution {
    fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs[0].clone();
        for v in strs.iter() {
            while v.find(prefix.as_str()) != Some(0) {
                prefix.pop();
                if prefix.is_empty() {
                    return "".to_string();
                }
            }
        }
        prefix
    }
}
