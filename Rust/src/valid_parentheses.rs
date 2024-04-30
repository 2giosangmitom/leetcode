pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::<char>::new();
    for char in s.chars() {
        match char {
            '{' | '(' | '[' => stack.push(char),
            '}' if stack.last() == Some(&'{') => {
                stack.pop();
            }
            ')' if stack.last() == Some(&'(') => {
                stack.pop();
            }
            ']' if stack.last() == Some(&'[') => {
                stack.pop();
            }
            _ => return false,
        }
    }
    stack.is_empty()
}
