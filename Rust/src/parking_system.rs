pub struct ParkingSystem {
    pub small: i32,
    pub medium: i32,
    pub big: i32,
}

impl ParkingSystem {
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem { small, medium, big }
    }

    pub fn add_car(&mut self, car_type: i32) -> bool {
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
