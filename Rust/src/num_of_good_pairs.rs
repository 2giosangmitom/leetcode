/**
 * Runtime: 0ms (Beats 100%)
 * Memory: 2MB (Beats 53.45%)
 */

pub struct Solution;

pub trait NumOfGoodPairs {
    fn num_identical_pairs(nums: Vec<i32>) -> i32;
}

impl NumOfGoodPairs for Solution {
    fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut result = 0;
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        for num in nums.into_iter() {
            if hash_map.contains_key(&num) {
                result += *hash_map.get_mut(&num).unwrap();
                *hash_map.get_mut(&num).unwrap() += 1;
            } else {
                hash_map.insert(num, 1);
            }
        }
        result
    }
}
