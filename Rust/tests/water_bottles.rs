#[cfg(test)]
mod tests {
    use leetcode::water_bottles::*;

    #[test]
    fn test_water_bottles_case1() {
        let num_bottles = 9;
        let num_exchange = 3;
        let want = 13;

        let result = num_water_bottles(num_bottles, num_exchange);

        assert_eq!(result, want);
    }

    #[test]
    fn test_water_bottles_case2() {
        let num_bottles = 15;
        let num_exchange = 4;
        let want = 19;

        let result = num_water_bottles(num_bottles, num_exchange);

        assert_eq!(result, want);
    }
}
