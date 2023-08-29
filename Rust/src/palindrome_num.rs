pub struct Solution;

pub trait PalindromeNumber {
    fn reverse_number(n: &i32) -> i32;

    fn is_palindrome(x: i32) -> bool;
}

impl PalindromeNumber for Solution {
    fn reverse_number(n: &i32) -> i32 {
        let mut num: i32 = 0;
        let mut temp_num: i32 = *n;
        while temp_num != 0 {
            let last_num: i32 = temp_num % 10;
            num = num * 10 + last_num;
            temp_num /= 10;
        }
        num
    }

    fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let reversed_number: i32 = Self::reverse_number(&x);
        x == reversed_number
    }
}

#[test]
fn test_palindrome_number() {
    struct Tt {
        num: i32,
        want: bool,
    }

    let cases: Vec<Tt> = vec![
        Tt { num: -10, want: false },
        Tt { num: 5, want: true },
        Tt { num: 121, want: true },
        Tt { num: 321, want: false },
        Tt { num: 111, want: true },
    ];

    for t in cases.into_iter() {
        let result = Solution::is_palindrome(t.num);
        assert_eq!(result, t.want);
    }
}
