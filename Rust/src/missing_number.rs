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
        let l = nums.len();
        let mut sum = 0;
        let total = (l * (l + 1)) / 2;
        for num in nums.iter() {
            sum += num;
        }
        total as i32 - sum
    }
}
