struct Solution;

trait ClimbingStairs {
    fn climb_stairs(n: i32) -> i32;
}

impl ClimbingStairs for Solution {
    fn climb_stairs(n: i32) -> i32 {
        match n {
            1 | 2 => n,
            _ => {
                let (mut a, mut b) = (1, 2);
                let mut count_loop = 0;
                loop {
                    if count_loop >= n - 3 {
                        break;
                    }

                    let temp = b;
                    b += a;
                    a = temp;

                    count_loop += 1;
                }
                a + b
            }
        }
    }
}

#[test]
fn test_climbing_stairs() {
    struct Tt {
        n: i32,
        want: i32,
    }

    let cases: Vec<Tt> = vec![
        Tt { n: 1, want: 1 },
        Tt { n: 2, want: 2 },
        Tt { n: 3, want: 3 },
        Tt { n: 4, want: 5 },
    ];

    for t in cases.into_iter() {
        let result = Solution::climb_stairs(t.n);
        assert_eq!(result, t.want);
    }
}
