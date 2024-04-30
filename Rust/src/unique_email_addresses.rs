pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    let mut unique_emails = Vec::<String>::new();
    for email in emails.into_iter() {
        let part = email.split('@').collect::<Vec<&str>>();
        let mut local_name = part[0].to_string();
        let domain_name = part[1];
        local_name = local_name.replace('.', "");
        if local_name.contains('+') {
            local_name = local_name.split('+').collect::<Vec<&str>>()[0].to_string();
        }
        let clean_email = local_name + "@" + domain_name;
        if !unique_emails.contains(&clean_email) {
            unique_emails.push(clean_email);
        }
    }
    unique_emails.len() as i32
}

pub fn num_unique_emails2(emails: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let mut hash_set = HashSet::<String>::new();
    for email in emails.into_iter() {
        let mut clean_email = String::new();
        let part = email.split('@').collect::<Vec<&str>>();
        let local_name = part[0];
        let domain_name = part[1];
        for v in local_name.chars() {
            match v {
                '+' => break,
                '.' => continue,
                _ => clean_email.push(v),
            }
        }
        hash_set.insert(clean_email + "@" + domain_name);
    }
    hash_set.len() as i32
}
