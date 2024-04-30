pub fn longest_common_prefix(strs: Vec<String>) -> String {
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
