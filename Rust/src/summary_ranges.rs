#[allow(dead_code)]
fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    let mut i = 0;
    while i < nums.len() {
        let mut end = i;
        while end + 1 < nums.len() && nums[end + 1] - nums[end] == 1 {
            end += 1;
        }

        if end > i {
            result.push(format!("{}->{}", nums[i], nums[end]));
        } else {
            result.push(format!("{}", nums[end]));
        }

        i = end + 1;
    }

    result
}

#[test]
fn test_summary_ranges() {
    struct Tt {
        nums: Vec<i32>,
        want: Vec<String>,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            nums: vec![0, 1, 2, 4, 5, 7],
            want: vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()],
        },
        Tt {
            nums: vec![0, 2, 3, 4, 6, 8, 9],
            want: vec![
                "0".to_string(),
                "2->4".to_string(),
                "6".to_string(),
                "8->9".to_string(),
            ],
        },
    ];

    for t in cases.into_iter() {
        let result = summary_ranges(t.nums);
        assert_eq!(result, t.want);
    }
}
