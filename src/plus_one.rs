struct Solution;

trait PlusOne {
    fn plus_one(digits: Vec<i32>) -> Vec<i32>;
}

impl PlusOne for Solution {
    fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for num in digits.iter_mut().rev() {
            match *num == 9 {
                true => *num = 0,
                false => {
                    *num += 1;
                    return digits;
                }
            }
        }
        digits.insert(0, 1);
        digits
    }
}

#[test]
fn test_plus_one() {
    struct Tt {
        digits: Vec<i32>,
        want: Vec<i32>,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            digits: vec![1, 2, 3],
            want: vec![1, 2, 4],
        },
        Tt {
            digits: vec![4, 3, 2, 1],
            want: vec![4, 3, 2, 2],
        },
        Tt { digits: vec![9], want: vec![1, 0] },
    ];

    for t in cases.into_iter() {
        let result = Solution::plus_one(t.digits);
        assert_eq!(result, t.want);
    }
}
