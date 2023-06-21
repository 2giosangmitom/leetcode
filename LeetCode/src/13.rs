struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
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
            }
            if number * 4 < result {
                result -= number;
            } else {
                result += number;
            }
        }
        result
    }
}
