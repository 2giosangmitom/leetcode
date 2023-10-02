struct Solution;

trait LongestCommonPrefix {
    fn longest_common_prefix(strs: Vec<String>) -> String;
}

impl LongestCommonPrefix for Solution {
    fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs.first().unwrap().to_string();
        for v in strs.iter().skip(1) {
            while v.find(prefix.as_str()) != Some(0) {
                prefix.pop();
                if prefix.is_empty() {
                    return "".to_string();
                }
            }
        }
        prefix.to_string()
    }
}

#[test]
fn test_longest_common_prefix() {
    struct Tt {
        strs: Vec<String>,
        want: String,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            strs: vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string(),
            ],
            want: "fl".to_string(),
        },
        Tt {
            strs: vec!["dog".to_string(), "racecar".to_string(), "car".to_string()],
            want: "".to_string(),
        },
    ];

    for t in cases.into_iter() {
        let result = Solution::longest_common_prefix(t.strs);
        assert_eq!(result, t.want);
    }
}
