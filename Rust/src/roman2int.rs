struct Solution;

trait RomanToInt {
    fn roman_to_int(s: String) -> i32;
}

impl RomanToInt for Solution {
    fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;
        let roman_map = HashMap::<char, i32>::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let mut result = 0;

        for char in s.chars().rev() {
            let number = roman_map.get(&char);

            if let Some(&value) = number {
                if value * 4 < result {
                    result -= value;
                } else {
                    result += value;
                }
            } else {
                return -1;
            }
        }
        result
    }
}

#[test]
fn test_roman_to_integer() {
    struct Tt {
        roman: String,
        want: i32,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            roman: "III".to_string(),
            want: 3,
        },
        Tt {
            roman: "LVIII".to_string(),
            want: 58,
        },
        Tt {
            roman: "MCMXCIV".to_string(),
            want: 1994,
        },
        Tt {
            roman: "XXIV".to_string(),
            want: 24,
        },
    ];

    for t in cases.into_iter() {
        let result: i32 = Solution::roman_to_int(t.roman);
        assert_eq!(result, t.want);
    }
}
