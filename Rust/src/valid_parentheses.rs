struct Solution;

trait ValidParentheses {
    fn is_valid(s: String) -> bool;
}

impl ValidParentheses for Solution {
    fn is_valid(s: String) -> bool {
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
}

#[test]
fn test_valid_parentheses() {
    struct Tt {
        s: String,
        want: bool,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            s: "()".to_string(),
            want: true,
        },
        Tt {
            s: "()[]{}".to_string(),
            want: true,
        },
        Tt {
            s: "(]".to_string(),
            want: false,
        },
    ];

    for t in cases.into_iter() {
        let result = Solution::is_valid(t.s);
        assert_eq!(result, t.want)
    }
}
