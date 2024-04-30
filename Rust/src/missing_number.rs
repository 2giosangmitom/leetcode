pub fn missing_number(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let total = (nums.len() * (nums.len() + 1) / 2) as i32;
    total - sum
}
