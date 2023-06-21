struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self { big, medium, small }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        if car_type == 1 && self.big > 0 {
            self.big -= 1;
            true
        } else if car_type == 2 && self.medium > 0 {
            self.medium -= 1;
            true
        } else if car_type == 3 && self.small > 0 {
            self.small -= 1;
            true
        } else {
            false
        }
    }
}
