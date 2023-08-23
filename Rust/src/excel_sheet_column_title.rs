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
