struct Solution;

trait SearchInsertPosition {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32;
}

impl SearchInsertPosition for Solution {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;
        let (mut left, mut right) = (0, (nums.len() - 1) as i32);
        while left <= right {
            let mid = left + (right - left) / 2;
            match nums[mid as usize].cmp(&target) {
                Ordering::Greater => right = mid - 1,
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return mid,
            }
        }
        left
    }
}

#[test]
fn test_search_insert_pos() {
    struct Tt {
        nums: Vec<i32>,
        target: i32,
        want: i32,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            nums: vec![1, 3, 5, 6],
            target: 5,
            want: 2,
        },
        Tt {
            nums: vec![1, 3, 5, 6],
            target: 2,
            want: 1,
        },
        Tt {
            nums: vec![1, 3, 5, 6],
            target: 7,
            want: 4,
        },
        Tt {
            nums: vec![1, 3, 5, 6],
            target: 0,
            want: 0,
        },
    ];

    for t in cases.into_iter() {
        let result = Solution::search_insert(t.nums, t.target);
        assert_eq!(result, t.want);
    }
}
