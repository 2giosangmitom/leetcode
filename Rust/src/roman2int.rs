struct Solution;

trait RomanToInt {
    fn roman_to_int(s: String) -> i32;
}

impl RomanToInt for Solution {
    fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut number: i32;
        for char in s.chars().rev() {
            match char {
                'I' => number = 1,
                'V' => number = 5,
                'X' => number = 10,
                'L' => number = 50,
                'C' => number = 100,
                'D' => number = 500,
                'M' => number = 1000,
                _ => panic!("Invalid roman number"),
            };
            if number * 4 < result {
                result -= number;
            } else {
                result += number;
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
