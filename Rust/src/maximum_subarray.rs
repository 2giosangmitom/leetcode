pub fn max_sub_array(nums: Vec<i32>) -> i32 {
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
