struct Solution;

trait MergeSortedArray {
    fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32);
}

impl MergeSortedArray for Solution {
    fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
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
}

#[test]
fn test_merge_sorted_array() {
    struct Tt {
        nums1: Vec<i32>,
        m: i32,
        nums2: Vec<i32>,
        n: i32,
        want: Vec<i32>,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            nums1: vec![1, 2, 3, 0, 0, 0],
            m: 3,
            nums2: vec![2, 5, 6],
            n: 3,
            want: vec![1, 2, 2, 3, 5, 6],
        },
        Tt {
            nums1: vec![1],
            m: 1,
            nums2: vec![],
            n: 0,
            want: vec![1],
        },
        Tt {
            nums1: vec![0],
            m: 0,
            nums2: vec![1],
            n: 1,
            want: vec![1],
        },
    ];

    for mut t in cases.into_iter() {
        Solution::merge(&mut t.nums1, t.m, &mut t.nums2, t.n);
        assert_eq!(t.nums1, t.want);
    }
}
