pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for num in digits.iter_mut().rev() {
        match *num == 9 {
            true => *num = 0,
            false => {
                *num += 1;
                return digits;
            }
        }
    }
    digits.insert(0, 1);
    digits
}
