#[cfg(test)]
mod tests {
    use leetcode::roman2int::*;

    #[test]
    fn test_roman_to_integer_case1() {
        let roman = "III".to_string();
        let want = 3;

        let result = roman_to_int(roman);

        assert_eq!(result, want);
    }

    #[test]
    fn test_roman_to_integer_case2() {
        let roman = "LVIII".to_string();
        let want = 58;

        let result = roman_to_int(roman);

        assert_eq!(result, want);
    }

    #[test]
    fn test_roman_to_integer_case3() {
        let roman = "MCMXCIV".to_string();
        let want = 1994;

        let result = roman_to_int(roman);

        assert_eq!(result, want);
    }

    #[test]
    fn test_roman_to_integer_case4() {
        let roman = "XXIV".to_string();
        let want = 24;

        let result = roman_to_int(roman);

        assert_eq!(result, want);
    }

    #[test]
    fn test_roman_to_integer_case5() {
        let roman = "LLVMR".to_string();
        let want = -1;

        let result = roman_to_int(roman);

        assert_eq!(result, want);
    }
}
