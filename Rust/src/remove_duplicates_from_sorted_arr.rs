pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k = 0;
    for i in 1..nums.len() {
        if nums[i] != nums[k] {
            k += 1;
            nums[k] = nums[i];
        }
    }
    (k + 1) as i32
}