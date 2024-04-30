pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (mut i, mut j, mut k) = (m - 1, n - 1, m + n - 1);
    while j >= 0 && k >= 0 {
        if i >= 0 && (nums1[i as usize] > nums2[j as usize]) {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
        k -= 1;
    }
}
