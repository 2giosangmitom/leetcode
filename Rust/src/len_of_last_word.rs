struct Solution;

trait LenOfLastWord {
    fn length_of_last_word(s: String) -> i32;
}

impl LenOfLastWord for Solution {
    fn length_of_last_word(s: String) -> i32 {
        let mut count = 0;
        for i in s.trim().chars().rev() {
            if i.is_whitespace() {
                break;
            }
            count += 1;
        }
        count
    }
}

#[test]
fn test_length_of_last_word() {
    struct Tt {
        s: String,
        want: i32,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            s: "Hello World".to_string(),
            want: 5,
        },
        Tt {
            s: "   fly me   to   the moon  ".to_string(),
            want: 4,
        },
        Tt {
            s: "luffy is still joyboy".to_string(),
            want: 6,
        },
    ];

    for t in cases.into_iter() {
        let result = Solution::length_of_last_word(t.s);
        assert_eq!(result, t.want);
    }
}
