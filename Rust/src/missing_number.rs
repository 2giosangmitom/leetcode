/**
 * Runtime: 2ms (Beats 83.51%)
 * Memory: 2.1MB (Beats 97.94%)
 */

pub struct Solution;

pub trait MissingNumber {
    fn missing_number(nums: Vec<i32>) -> i32;
}

impl MissingNumber for Solution {
    fn missing_number(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let total = ((nums.len() * (nums.len() + 1)) / 2) as i32;
        total - sum
    }
}
