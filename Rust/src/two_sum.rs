/**
 * Runtime: 2ms (Beats 82.51%)
 * Memory: 2.4MB (Beats 40.3%)
 */

pub struct Solution;

pub trait TwoSum {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>;
}

impl TwoSum for Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            let need_number: i32 = target - *v;
            if let Some(&value) = hash_map.get(&need_number) {
                return vec![value, i as i32];
            } else {
                hash_map.insert(*v, i as i32);
            }
        }
        vec![-1]
    }
}
