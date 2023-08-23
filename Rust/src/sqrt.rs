pub struct Solution;

pub trait Sqrt {
    fn my_sqrt(x: i32) -> i32;
}

impl Sqrt for Solution {
    fn my_sqrt(x: i32) -> i32 {
        use std::cmp::Ordering;
        let (mut left, mut right) = (1, x);
        while left <= right {
            let mid = left + (right - left) / 2;
            match mid.cmp(&(x / mid)) {
                Ordering::Equal => return mid,
                Ordering::Greater => right = mid - 1,
                Ordering::Less => left = mid + 1,
            }
        }
        left - 1
    }
}
