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
