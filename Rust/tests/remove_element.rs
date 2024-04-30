#[cfg(test)]
mod tests {
    use leetcode::remove_element::*;

    #[test]
    fn test_remove_element_case1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let want = 2;
        let want_nums = vec![2, 2];

        let result = remove_element(&mut nums, val);

        assert_eq!(result, want);
        assert_eq!(&nums[..want as usize], &want_nums);
    }

    #[test]
    fn test_remove_element_case2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        let want = 5;
        let want_nums = vec![0, 1, 3, 0, 4];

        let result = remove_element(&mut nums, val);

        assert_eq!(result, want);
        assert_eq!(&nums[..want as usize], &want_nums);
    }
}
