pub fn length_of_last_word(s: String) -> i32 {
    let mut count = 0;
    for i in s.trim().chars().rev() {
        if i.is_whitespace() {
            break;
        }
        count += 1;
    }
    count
}
