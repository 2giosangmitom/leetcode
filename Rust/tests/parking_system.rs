#[cfg(test)]
mod tests {
    use leetcode::parking_system::*;

    #[test]
    fn test_parking_system_case1() {
        let mut obj = ParkingSystem::new(1, 1, 0);
        let add_car = vec![1, 2, 3, 1];
        let want = vec![true, true, false, false];
        let mut result = vec![];

        for &car in &add_car {
            let add = obj.add_car(car);
            result.push(add);
        }

        assert_eq!(result, want);
    }

    #[test]
    fn test_parking_system_case2() {
        let mut obj = ParkingSystem::new(2, 15, 44);
        let add_car = vec![1, 1, 2, 1, 3, 3, 1, 2, 2, 3, 1];
        let want = vec![
            true, true, true, false, true, true, false, true, true, true, false,
        ];
        let mut result = vec![];

        for &car in &add_car {
            let add = obj.add_car(car);
            result.push(add);
        }

        assert_eq!(result, want);
    }
}
