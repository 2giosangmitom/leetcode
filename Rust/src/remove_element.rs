pub struct Solution;

pub trait RemoveElement {
    fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32;
}

impl RemoveElement for Solution {
    fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }
}

#[test]
fn test_remove_element() {
    struct Tt {
        nums: Vec<i32>,
        val: i32,
        want: i32,
        want_nums: Vec<i32>,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            nums: vec![3, 2, 2, 3],
            val: 3,
            want: 2,
            want_nums: vec![2, 2],
        },
        Tt {
            nums: vec![0, 1, 2, 2, 3, 0, 4, 2],
            val: 2,
            want: 5,
            want_nums: vec![0, 1, 3, 0, 4],
        },
    ];

    for mut t in cases.into_iter() {
        let result = Solution::remove_element(&mut t.nums, t.val);
        assert_eq!(result, t.want);
        assert_eq!(t.nums[..t.want as usize], t.want_nums);
    }
}
