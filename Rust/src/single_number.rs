#[allow(dead_code)]
fn single_number(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut hash_map = HashMap::<i32, i32>::new();
    nums.clone().into_iter().for_each(|num| {
        if hash_map.get(&num).is_none() {
            hash_map.insert(num, 1);
        } else {
            *hash_map.get_mut(&num).unwrap() += 1;
        }
    });

    let mut result = 0;
    for num in nums.into_iter() {
        let v = hash_map.get(&num).unwrap();
        if *v == 1 {
            result = num;
        }
    }
    result
}

#[test]
fn test_single_number() {
    struct Tt {
        nums: Vec<i32>,
        want: i32,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            nums: vec![2, 2, 1],
            want: 1,
        },
        Tt {
            nums: vec![4, 1, 2, 1, 2],
            want: 4,
        },
        Tt {
            nums: vec![1],
            want: 1,
        },
    ];

    for t in cases.into_iter() {
        let result = single_number(t.nums);
        assert_eq!(result, t.want);
    }
}
