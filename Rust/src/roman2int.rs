pub fn roman_to_int(s: String) -> i32 {
    use std::collections::HashMap;
    let roman_map = HashMap::<char, i32>::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut result = 0;

    for char in s.chars().rev() {
        let number = roman_map.get(&char);

        if let Some(&value) = number {
            if value * 4 < result {
                result -= value;
            } else {
                result += value;
            }
        } else {
            return -1;
        }
    }
    result
}
