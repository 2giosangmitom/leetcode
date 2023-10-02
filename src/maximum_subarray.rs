struct Solution;

trait MaximumSubArray {
    fn max_sub_array(nums: Vec<i32>) -> i32;
}

impl MaximumSubArray for Solution {
    fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut temp = nums[0];
        for num in nums.into_iter().skip(1) {
            temp += num;
            if temp < num {
                temp = num;
            }
            if temp > result {
                result = temp;
            }
        }
        result
    }
}

#[test]
fn test_maximum_subarray() {
    struct Tt {
        nums: Vec<i32>,
        want: i32,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            nums: vec![-2, 1, -3, 4, -1, 2, 1, -5, 4],
            want: 6,
        },
        Tt {
            nums: vec![0],
            want: 0,
        },
        Tt {
            nums: vec![5, 4, -1, 7, 8],
            want: 23,
        },
    ];

    for t in cases.into_iter() {
        let result = Solution::max_sub_array(t.nums);
        assert_eq!(result, t.want);
    }
}
