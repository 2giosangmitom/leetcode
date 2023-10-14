struct Solution;

trait RemoveDuplicates {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32;
}

impl RemoveDuplicates for Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[k] {
                k += 1;
                nums[k] = nums[i];
            }
        }
        (k + 1) as i32
    }
}

#[test]
fn test_remove_duplicates_from_sorted_array() {
    struct Tt {
        nums: Vec<i32>,
        want: i32,
        want_nums: Vec<i32>,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            nums: vec![1, 1, 2],
            want: 2,
            want_nums: vec![1, 2],
        },
        Tt {
            nums: vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
            want: 5,
            want_nums: vec![0, 1, 2, 3, 4],
        },
    ];

    for mut t in cases.into_iter() {
        let result = Solution::remove_duplicates(&mut t.nums);
        assert_eq!(result, t.want);
        assert_eq!(t.nums[..t.want as usize], t.want_nums);
    }
}
