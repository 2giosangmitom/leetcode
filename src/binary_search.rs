struct Solution;

trait BinarySearch {
    fn search(nums: Vec<i32>, target: i32) -> i32;
}

impl BinarySearch for Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;
        let (mut left, mut right) = (0, nums.len() - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Greater => right = mid - 1,
                Ordering::Less => left = mid + 1,
            }
        }
        -1
    }
}

#[test]
fn test_binary_search() {
    struct Tt {
        nums: Vec<i32>,
        target: i32,
        want: i32,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            nums: vec![-1, 0, 3, 5, 9, 12],
            target: 9,
            want: 4,
        },
        Tt {
            nums: vec![-1, 0, 3, 5, 9, 12],
            target: 2,
            want: -1,
        },
    ];

    for t in cases.into_iter() {
        let result = Solution::search(t.nums, t.target);
        assert_eq!(result, t.want);
    }
}
