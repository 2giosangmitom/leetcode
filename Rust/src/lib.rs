mod best_time2buy_n_sell_stock;
mod binary_search;
mod len_of_last_word;
mod longest_common_prefix;
mod missing_number;
mod num_of_good_pairs;
mod palindrome_num;
mod parking_system;
mod roman2int;
mod two_sum;
mod unique_email_addresses;
mod valid_parentheses;
mod water_bottles;

#[cfg(test)]
mod tests {
    use crate::{
        best_time2buy_n_sell_stock::{self, BestTimeToBuyAndSellStock},
        binary_search::{self, BinarySearch},
        len_of_last_word::{self, LenOfLastWord},
        longest_common_prefix::{self, LongestCommonPrefix},
        missing_number::{self, MissingNumber},
        num_of_good_pairs::{self, NumOfGoodPairs},
        palindrome_num::{self, PalindromeNumber},
        parking_system::{self, DesignParkingSystem},
        roman2int::{self, RomanToInt},
        two_sum::{self, TwoSum},
        unique_email_addresses::{self, UniqueEmailAddresses},
        valid_parentheses::{self, ValidParentheses},
        water_bottles::{self, WaterBottles},
    };

    #[test]
    fn test_1() {
        struct Tt {
            nums: Vec<i32>,
            target: i32,
            want: Vec<i32>,
        }

        let cases: Vec<Tt> = vec![
            Tt {
                nums: vec![2, 7, 11, 15],
                target: 9,
                want: vec![0, 1],
            },
            Tt {
                nums: vec![3, 2, 4],
                target: 6,
                want: vec![1, 2],
            },
            Tt {
                nums: vec![3, 3],
                target: 6,
                want: vec![0, 1],
            },
            Tt {
                nums: vec![2, 3, 4, 1, 25, 8],
                target: 30,
                want: vec![-1],
            },
        ];

        for t in cases.into_iter() {
            let result = two_sum::Solution::two_sum(t.nums, t.target);
            assert_eq!(result, t.want);
        }
    }

    #[test]
    fn test_9() {
        struct Tt {
            num: i32,
            want: bool,
        }

        let cases: Vec<Tt> = vec![
            Tt { num: -10, want: false },
            Tt { num: 5, want: true },
            Tt { num: 121, want: true },
        ];

        for t in cases.into_iter() {
            let result = palindrome_num::Solution::is_palindrome(t.num);
            assert_eq!(result, t.want);
        }
    }

    #[test]
    fn test_13() {
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
        ];

        for t in cases.into_iter() {
            let result: i32 = roman2int::Solution::roman_to_int(t.roman);
            assert_eq!(result, t.want);
        }
    }

    #[test]
    fn test_14() {
        struct Tt {
            strs: Vec<String>,
            want: String,
        }

        let cases: Vec<Tt> = vec![
            Tt {
                strs: vec!["flower".to_string(), "flow".to_string(), "flight".to_string()],
                want: "fl".to_string(),
            },
            Tt {
                strs: vec!["dog".to_string(), "racecar".to_string(), "car".to_string()],
                want: "".to_string(),
            },
        ];

        for t in cases.into_iter() {
            let result = longest_common_prefix::Solution::longest_common_prefix(t.strs);
            assert_eq!(result, t.want);
        }
    }

    #[test]
    fn test_20() {
        struct Tt {
            s: String,
            want: bool,
        }

        let cases: Vec<Tt> = vec![
            Tt {
                s: "()".to_string(),
                want: true,
            },
            Tt {
                s: "()[]{}".to_string(),
                want: true,
            },
            Tt {
                s: "(]".to_string(),
                want: false,
            },
        ];

        for t in cases.into_iter() {
            let result = valid_parentheses::Solution::is_valid(t.s);
            assert_eq!(result, t.want)
        }
    }

    #[test]
    fn test_704() {
        struct Tt {
            nums: Vec<i32>,
            target: i32,
            want: i32,
        }

        let cases: Vec<Tt> = vec![
            Tt {
                nums: vec![-1, 0, 3, 5, 9, 12],
                target: 9,
                want: 4,
            },
            Tt {
                nums: vec![-1, 0, 3, 5, 9, 12],
                target: 2,
                want: -1,
            },
        ];

        for t in cases.into_iter() {
            let result = binary_search::Solution::search(t.nums, t.target);
            assert_eq!(result, t.want);
        }
    }

    #[test]
    fn test_268() {
        struct Tt {
            nums: Vec<i32>,
            want: i32,
        }

        let cases: Vec<Tt> = vec![
            Tt {
                nums: vec![3, 0, 1],
                want: 2,
            },
            Tt {
                nums: vec![0, 1],
                want: 2,
            },
            Tt {
                nums: vec![9, 6, 4, 2, 3, 5, 7, 0, 1],
                want: 8,
            },
        ];

        for t in cases.into_iter() {
            let result = missing_number::Solution::missing_number(t.nums);
            assert_eq!(result, t.want);
        }
    }

    #[test]
    fn test_121() {
        struct Tt {
            prices: Vec<i32>,
            want: i32,
        }

        let cases: Vec<Tt> = vec![
            Tt {
                prices: vec![7, 1, 5, 3, 6, 4],
                want: 5,
            },
            Tt {
                prices: vec![7, 6, 4, 3, 1],
                want: 0,
            },
            Tt {
                prices: vec![2, 1, 4],
                want: 3,
            },
        ];

        for t in cases.into_iter() {
            let result = best_time2buy_n_sell_stock::Solution::max_profit(t.prices);
            assert_eq!(result, t.want);
        }
    }

    #[test]
    fn test_929() {
        struct Tt {
            emails: Vec<String>,
            want: i32,
        }

        let cases: Vec<Tt> = vec![
            Tt {
                emails: vec![
                    "test.email+alex@leetcode.com".to_string(),
                    "test.e.mail+bob.cathy@leetcode.com".to_string(),
                    "testemail+david@lee.tcode.com".to_string(),
                ],
                want: 2,
            },
            Tt {
                emails: vec![
                    "a@leetcode.com".to_string(),
                    "b@leetcode.com".to_string(),
                    "c@leetcode.com".to_string(),
                ],
                want: 3,
            },
        ];

        for t in cases.into_iter() {
            let result = unique_email_addresses::Solution::num_unique_emails(t.emails);
            assert_eq!(result, t.want);
        }
    }

    #[test]
    fn test_58() {
        struct Tt {
            s: String,
            want: i32,
        }

        let cases: Vec<Tt> = vec![
            Tt {
                s: "Hello World".to_string(),
                want: 5,
            },
            Tt {
                s: "   fly me   to   the moon  ".to_string(),
                want: 4,
            },
            Tt {
                s: "luffy is still joyboy".to_string(),
                want: 6,
            },
        ];

        for t in cases.into_iter() {
            let result = len_of_last_word::Solution::length_of_last_word(t.s);
            assert_eq!(result, t.want);
        }
    }

    #[test]
    fn test_1518() {
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
            let result = water_bottles::Solution::num_water_bottles(t.num_bottles, t.num_exchange);
            assert_eq!(result, t.want);
        }
    }

    #[test]
    fn test_1603() {
        struct Tt {
            big: i32,
            medium: i32,
            small: i32,
            add_car: Vec<i32>,
            want: Vec<bool>,
        }

        let cases: Vec<Tt> = vec![
            Tt {
                big: 1,
                medium: 1,
                small: 0,
                add_car: vec![1, 2, 3, 1],
                want: vec![true, true, false, false],
            },
            Tt {
                big: 2,
                medium: 15,
                small: 44,
                add_car: vec![1, 1, 2, 1, 3, 3, 1, 2, 2, 3, 1],
                want: vec![true, true, true, false, true, true, false, true, true, true, false],
            },
        ];

        for t in cases.into_iter() {
            let mut obj = parking_system::ParkingSystem::new(t.big, t.medium, t.small);
            let mut result = vec![];
            for v in t.add_car.into_iter() {
                let add = obj.add_car(v);
                result.push(add);
            }
            assert_eq!(result, t.want);
        }
    }

    #[test]
    fn test_1512() {
        struct Tt {
            nums: Vec<i32>,
            want: i32,
        }

        let cases: Vec<Tt> = vec![
            Tt {
                nums: vec![1, 2, 3, 1, 1, 3],
                want: 4,
            },
            Tt {
                nums: vec![1, 1, 1, 1],
                want: 6,
            },
            Tt {
                nums: vec![1, 2, 3],
                want: 0,
            },
        ];

        for t in cases.into_iter() {
            let result = num_of_good_pairs::Solution::num_identical_pairs(t.nums);
            assert_eq!(result, t.want);
        }
    }
}
