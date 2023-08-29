pub struct Solution;

pub trait ExcelSheetColumnTitle {
    fn convert_to_title(column_number: i32) -> String;
}

impl ExcelSheetColumnTitle for Solution {
    fn convert_to_title(mut column_number: i32) -> String {
        let mut result = Vec::new();
        while column_number > 0 {
            column_number -= 1;
            result.push(b'A' + (column_number % 26) as u8);
            column_number /= 26;
        }
        result.reverse();
        String::from_utf8(result).expect("Failed to convert vector of bytes to String")
    }
}

#[test]
fn test_excel_sheet_column_title() {
    struct Tt {
        column_number: i32,
        want: String,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            column_number: 1,
            want: "A".to_string(),
        },
        Tt {
            column_number: 28,
            want: "AB".to_string(),
        },
        Tt {
            column_number: 701,
            want: "ZY".to_string(),
        },
    ];

    for t in cases.into_iter() {
        let result = Solution::convert_to_title(t.column_number);
        assert_eq!(result, t.want);
    }
}
