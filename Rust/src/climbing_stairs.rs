pub fn climb_stairs(n: i32) -> i32 {
    match n {
        1 | 2 => n,
        _ => {
            let (mut a, mut b) = (1, 2);
            let mut count_loop = 0;
            loop {
                if count_loop >= n - 3 {
                    break;
                }

                let temp = b;
                b += a;
                a = temp;

                count_loop += 1;
            }
            a + b
        }
    }
}
