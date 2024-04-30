pub fn count_odds(low: i32, high: i32) -> i32 {
    let length = high - low + 1;
    let mut result = length / 2;
    if low % 2 == 1 && high % 2 == 1 {
        result += 1;
    }
    result
}
