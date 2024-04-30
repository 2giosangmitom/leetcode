#[cfg(test)]
mod tests {
    use leetcode::count_odd_number::*;

    #[test]
    fn test_count_odds_in_range_from_8_to_10() {
        assert_eq!(1, count_odds(8, 10));
    }

    #[test]
    fn test_count_odds_in_range_from_3_to_6() {
        assert_eq!(2, count_odds(3, 6));
    }

    #[test]
    fn test_count_odds_in_range_from_8_to_13() {
        assert_eq!(3, count_odds(8, 13));
    }

    #[test]
    fn test_count_odds_in_range_from_3_to_7() {
        assert_eq!(3, count_odds(3, 7));
    }

    #[test]
    fn test_count_odds_in_range_from_2_to_6() {
        assert_eq!(2, count_odds(2, 6));
    }
}
