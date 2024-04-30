pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    let mut i = 0;
    while i < nums.len() {
        let mut end = i;
        while end + 1 < nums.len() && nums[end + 1] - nums[end] == 1 {
            end += 1;
        }

        if end > i {
            result.push(format!("{}->{}", nums[i], nums[end]));
        } else {
            result.push(format!("{}", nums[end]));
        }

        i = end + 1;
    }

    result
}
