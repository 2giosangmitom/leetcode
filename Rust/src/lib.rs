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
        let result = two_sum::Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_9() {
        let result2 = palindrome_num::Solution::is_palindrome(121);
        assert_eq!(result2, true)
    }
}
