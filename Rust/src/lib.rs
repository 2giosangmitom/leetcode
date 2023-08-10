mod best_time2buy_n_sell_stock;
mod binary_search;
mod longest_common_prefix;
mod missing_number;
mod palindrome_num;
mod roman2int;
mod two_sum;
mod valid_parentheses;

#[cfg(test)]
mod tests {
    use crate::{
        best_time2buy_n_sell_stock::{self, BestTimeToBuyAndSellStock},
        binary_search::{self, BinarySearch},
        longest_common_prefix::{self, LongestCommonPrefix},
        missing_number::{self, MissingNumber},
        palindrome_num::{self, PalindromeNumber},
        roman2int::{self, RomanToInt},
        two_sum::{self, TwoSum},
        valid_parentheses::{self, ValidParentheses},
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

        for t in cases.iter() {
            let result: Vec<i32> = two_sum::Solution::two_sum(t.nums.clone(), t.target);
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

        for t in cases.iter() {
            let result: bool = palindrome_num::Solution::is_palindrome(t.num);
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

        for t in cases.iter() {
            let result: i32 = roman2int::Solution::roman_to_int(t.roman.clone());
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

        for t in cases.iter() {
            let result: String = longest_common_prefix::Solution::longest_common_prefix(t.strs.clone());
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

        for t in cases.iter() {
            let result = valid_parentheses::Solution::is_valid(t.s.clone());
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

        for t in cases.iter() {
            let result = binary_search::Solution::search(t.nums.clone(), t.target);
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

        for t in cases.iter() {
            let result = missing_number::Solution::missing_number(t.nums.clone());
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

        for t in cases.iter() {
            let result = best_time2buy_n_sell_stock::Solution::max_profit(t.prices.clone());
            assert_eq!(result, t.want);
        }
    }
}
