pub struct Solution;

pub trait MissingNumber {
    fn missing_number(nums: Vec<i32>) -> i32;
}

impl MissingNumber for Solution {
    fn missing_number(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let total = (nums.len() * (nums.len() + 1) / 2) as i32;
        total - sum
    }
}

#[test]
fn test_missing_number() {
    struct Tt {
        nums: Vec<i32>,
        want: i32,
    }

    let cases: Vec<Tt> = vec![
        Tt { nums: vec![3, 0, 1], want: 2 },
        Tt { nums: vec![0, 1], want: 2 },
        Tt {
            nums: vec![9, 6, 4, 2, 3, 5, 7, 0, 1],
            want: 8,
        },
    ];

    for t in cases.into_iter() {
        let result = Solution::missing_number(t.nums);
        assert_eq!(result, t.want);
    }
}
