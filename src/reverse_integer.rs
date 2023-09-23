struct Solution;

trait ReverseInteger {
    fn reverse(x: i32) -> i32;
}

impl ReverseInteger for Solution {
    fn reverse(mut x: i32) -> i32 {
        let mut result: i32 = 0;
        while x != 0 {
            match result.checked_mul(10) {
                Some(num) => {
                    let last_number = x % 10;
                    if let Some(ok) = num.checked_add(last_number) {
                        result = ok
                    } else {
                        return 0;
                    }
                }
                None => return 0,
            }
            x /= 10;
        }
        result
    }
}

#[test]
fn test_reverse_integer() {
    struct Tt {
        x: i32,
        want: i32,
    }

    let cases: Vec<Tt> = vec![
        Tt { x: 123, want: 321 },
        Tt { x: -123, want: -321 },
        Tt { x: 120, want: 21 },
        Tt { x: 1534236469, want: 0 },
        Tt { x: -2147483648, want: 0 },
        Tt { x: 900000, want: 9 },
    ];

    for t in cases.into_iter() {
        let result = Solution::reverse(t.x);
        assert_eq!(result, t.want);
    }
}
