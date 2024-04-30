use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map = HashMap::<i32, i32>::new();
    for (i, v) in nums.iter().enumerate() {
        let complement = target - v;
        if let Some(&value) = hash_map.get(&complement) {
            return vec![value, i as i32];
        } else {
            hash_map.insert(*v, i as i32);
        }
    }
    vec![-1]
}

pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, v) in nums.iter().enumerate() {
        let complement = target - v;
        for (j, n) in nums.iter().enumerate().skip(i + 1) {
            if complement == *n && i != j {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![-1]
}
