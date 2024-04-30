#[cfg(test)]
mod tests {
    use leetcode::merge_sorted_arr::*;

    #[test]
    fn test_merge_sorted_array_case1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        let want = vec![1, 2, 2, 3, 5, 6];

        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, want);
    }

    #[test]
    fn test_merge_sorted_array_case2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        let want = vec![1];

        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, want);
    }

    #[test]
    fn test_merge_sorted_array_case3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        let want = vec![1];

        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, want);
    }
}
