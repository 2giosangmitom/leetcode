use std::collections::HashMap;

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut hash_map = HashMap::<i32, i32>::new();
    nums.clone().into_iter().for_each(|num| {
        if hash_map.get(&num).is_none() {
            hash_map.insert(num, 1);
        } else {
            *hash_map.get_mut(&num).unwrap() += 1;
        }
    });

    let mut result = 0;
    for num in nums.into_iter() {
        let v = hash_map.get(&num).unwrap();
        if *v == 1 {
            result = num;
        }
    }
    result
}
