pub struct ParkingSystem {
    small: i32,
    medium: i32,
    big: i32,
}

pub trait DesignParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self;

    fn add_car(&mut self, car_type: i32) -> bool;
}

impl DesignParkingSystem for ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem { small, medium, big }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        if car_type == 1 && self.big > 0 {
            self.big -= 1;
            return true;
        }
        if car_type == 2 && self.medium > 0 {
            self.medium -= 1;
            return true;
        }
        if car_type == 3 && self.small > 0 {
            self.small -= 1;
            return true;
        }
        false
    }
}

#[test]
fn test_parking_system() {
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
        let mut obj = ParkingSystem::new(t.big, t.medium, t.small);
        let mut result = vec![];
        for v in t.add_car.into_iter() {
            let add = obj.add_car(v);
            result.push(add);
        }
        assert_eq!(result, t.want);
    }
}
