pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
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
