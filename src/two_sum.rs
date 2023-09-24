struct Solution;

trait TwoSum {
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

#[test]
fn test_two_sum() {
    struct Tt {
        nums: Vec<i32>,
        target: i32,
        want: Vec<i32>,
    }

    let cases: Vec<Tt> = vec![
        Tt { nums: vec![2, 7, 11, 15], target: 9, want: vec![0, 1] },
        Tt { nums: vec![3, 2, 4], target: 6, want: vec![1, 2] },
        Tt { nums: vec![3, 3], target: 6, want: vec![0, 1] },
        Tt { nums: vec![2, 3, 4, 1, 25, 8], target: 30, want: vec![-1] },
    ];

    for t in cases.into_iter() {
        let result = Solution::two_sum(t.nums, t.target);
        assert_eq!(result, t.want);
    }
}
