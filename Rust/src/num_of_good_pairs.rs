pub struct Solution;

pub trait NumOfGoodPairs {
    fn num_identical_pairs(nums: Vec<i32>) -> i32;
}

impl NumOfGoodPairs for Solution {
    fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        use std::collections::hash_map;
        use std::collections::HashMap;
        let mut result = 0;
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        for num in nums.into_iter() {
            if let hash_map::Entry::Vacant(e) = hash_map.entry(num) {
                e.insert(1);
            } else {
                result += *hash_map.get_mut(&num).unwrap();
                *hash_map.get_mut(&num).unwrap() += 1;
            }
        }
        result
    }
}
