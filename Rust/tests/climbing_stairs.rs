#[cfg(test)]
mod tests {
    use leetcode::climbing_stairs::*;

    #[test]
    fn test_climbing_stairs_1() {
        assert_eq!(climb_stairs(1), 1);
    }

    #[test]
    fn test_climbing_stairs_2() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn test_climbing_stairs_3() {
        assert_eq!(climb_stairs(3), 3);
    }

    #[test]
    fn test_climbing_stairs_4() {
        assert_eq!(climb_stairs(4), 5);
    }
}
