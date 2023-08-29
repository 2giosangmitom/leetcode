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

#[test]
fn test_number_of_good_pairs() {
    struct Tt {
        nums: Vec<i32>,
        want: i32,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            nums: vec![1, 2, 3, 1, 1, 3],
            want: 4,
        },
        Tt { nums: vec![1, 1, 1, 1], want: 6 },
        Tt { nums: vec![1, 2, 3], want: 0 },
    ];

    for t in cases.into_iter() {
        let result = Solution::num_identical_pairs(t.nums);
        assert_eq!(result, t.want);
    }
}
