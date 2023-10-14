struct Solution;

trait CountOdd {
    fn count_odds(low: i32, high: i32) -> i32;
}

impl CountOdd for Solution {
    fn count_odds(low: i32, high: i32) -> i32 {
        let length = high - low + 1;
        let mut result = length / 2;
        if low % 2 == 1 && high % 2 == 1 {
            result += 1;
        }
        result
    }
}

#[test]
fn test_count_odd_numbers() {
    struct Tt {
        low: i32,
        high: i32,
        want: i32,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            low: 8,
            high: 10,
            want: 1,
        },
        Tt {
            low: 3,
            high: 6,
            want: 2,
        },
        Tt {
            low: 8,
            high: 13,
            want: 3,
        },
        Tt {
            low: 3,
            high: 7,
            want: 3,
        },
        Tt {
            low: 2,
            high: 6,
            want: 2,
        },
    ];

    for t in cases.into_iter() {
        let result = Solution::count_odds(t.low, t.high);
        assert_eq!(result, t.want);
    }
}
