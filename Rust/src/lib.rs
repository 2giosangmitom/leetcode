mod palindrome_num;
mod two_sum;

#[cfg(test)]
mod tests {
    use crate::{
        palindrome_num::{self, PalindromeNumber},
        two_sum::{self, TwoSum},
    };

    #[test]
    fn test_1() {
        struct Tt {
            nums: Vec<i32>,
            target: i32,
            want: Vec<i32>,
        }

        #[rustfmt::skip]
        let cases: Vec<Tt> = vec![
            Tt { nums: vec![2, 7, 11, 15], target: 9, want: vec![0, 1] },
            Tt { nums: vec![3, 2, 4], target: 6, want: vec![1, 2] },
            Tt { nums: vec![3, 3], target: 6, want: vec![0, 1] },
            Tt { nums: vec![2, 3, 4, 1, 25, 8], target: 30, want: vec![-1] },
        ];

        for t in cases.iter() {
            let result: Vec<i32> = two_sum::Solution::two_sum(t.nums.clone(), t.target);
            assert_eq!(result, t.want);
        }
    }

    #[test]
    fn test_9() {
        struct Tt {
            num: i32,
            want: bool,
        }

        #[rustfmt::skip]
        let cases: Vec<Tt> = vec![
            Tt { num: -10, want: false },
            Tt { num: 5, want: true },
            Tt { num: 121, want: true }
        ];

        for t in cases.iter() {
            let result: bool = palindrome_num::Solution::is_palindrome(t.num);
            assert_eq!(result, t.want);
        }
    }
}
