/**
 * Runtime: 13ms (Beats 87.93%)
 * Memory: 2.6MB (Beats 18.97%)
 */

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
