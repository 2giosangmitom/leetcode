pub struct Solution;

pub trait RemoveDuplicates {
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
