pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
    let mut drank = 0;

    while num_bottles >= num_exchange {
        let rem = num_bottles % num_exchange;
        drank += num_bottles - rem;
        num_bottles = (num_bottles / num_exchange) + rem;
    }

    drank + num_bottles
}
