#[cfg(test)]
mod tests {
    use leetcode::sqrt::*;

    #[test]
    fn test_sqrt_case1() {
        let x = 4;
        let want = 2;

        let result = my_sqrt(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_sqrt_case2() {
        let x = 8;
        let want = 2;

        let result = my_sqrt(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_sqrt_case3() {
        let x = 1;
        let want = 1;

        let result = my_sqrt(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_sqrt_case4() {
        let x = 10;
        let want = 3; // The square root of 10 is between 3 and 4

        let result = my_sqrt(x);

        assert_eq!(result, want);
    }

    #[test]
    fn test_sqrt_case5() {
        let x = 10000;
        let want = 100; // The square root of 10000 is 100

        let result = my_sqrt(x);

        assert_eq!(result, want);
    }
}
