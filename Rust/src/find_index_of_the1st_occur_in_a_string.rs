struct Solution;

trait FindIndexOfTheFirstOccurInAString {
    fn str_str(haystack: String, needle: String) -> i32;
}

impl FindIndexOfTheFirstOccurInAString for Solution {
    fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_length = haystack.len();
        let needle_length = needle.len();
        if haystack_length < needle_length {
            return -1;
        } else {
            for i in 0..=haystack_length - needle_length {
                if haystack[i..i + needle_length] == needle {
                    return i as i32;
                }
            }
        }
        -1
    }
}

#[test]
fn test_find_index_of_the_first_occur_in_a_string() {
    struct Tt {
        haystack: String,
        needle: String,
        want: i32,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            haystack: "sadbutsad".to_string(),
            needle: "sad".to_string(),
            want: 0,
        },
        Tt {
            haystack: "leetcode".to_string(),
            needle: "leeto".to_string(),
            want: -1,
        },
        Tt {
            haystack: "hello".to_string(),
            needle: "ll".to_string(),
            want: 2,
        },
        Tt {
            haystack: "onepiece".to_string(),
            needle: "pheloiquaaeoi".to_string(),
            want: -1,
        },
    ];

    for t in cases.into_iter() {
        let result = Solution::str_str(t.haystack, t.needle);
        assert_eq!(result, t.want);
    }
}
