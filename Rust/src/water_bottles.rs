pub struct Solution;

pub trait WaterBottles {
    fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32;
}

impl WaterBottles for Solution {
    fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut drank = 0;

        while num_bottles >= num_exchange {
            let rem = num_bottles % num_exchange;
            drank += num_bottles - rem;
            num_bottles = (num_bottles / num_exchange) + rem;
        }

        drank + num_bottles
    }
}

#[test]
fn test_water_bottles() {
    struct Tt {
        num_bottles: i32,
        num_exchange: i32,
        want: i32,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            num_bottles: 9,
            num_exchange: 3,
            want: 13,
        },
        Tt {
            num_bottles: 15,
            num_exchange: 4,
            want: 19,
        },
    ];

    for t in cases.into_iter() {
        let result = Solution::num_water_bottles(t.num_bottles, t.num_exchange);
        assert_eq!(result, t.want);
    }
}
