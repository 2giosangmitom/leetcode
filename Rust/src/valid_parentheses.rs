pub struct Solution;

pub trait ValidParentheses {
    fn is_valid(s: String) -> bool;
}

impl ValidParentheses for Solution {
    fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
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
}
