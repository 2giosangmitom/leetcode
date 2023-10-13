struct Solution;

trait PalindromeNumber {
    fn is_palindrome(x: i32) -> bool;
}

impl PalindromeNumber for Solution {
    fn is_palindrome(x: i32) -> bool {
        let reverse_number = |n: &i32| -> i32 {
            let mut result = 0;
            let mut temp = *n;
            while temp != 0 {
                let last_digit = temp % 10;
                result = result * 10 + last_digit;
                temp /= 10;
            }
            result
        };

        if x < 0 {
            return false;
        }
        let reversed_number = reverse_number(&x);
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
        Tt {
            num: -10,
            want: false,
        },
        Tt { num: 5, want: true },
        Tt {
            num: 121,
            want: true,
        },
        Tt {
            num: 321,
            want: false,
        },
        Tt {
            num: 111,
            want: true,
        },
    ];

    for t in cases.into_iter() {
        let result = Solution::is_palindrome(t.num);
        assert_eq!(result, t.want);
    }
}
