pub struct Solution;

pub trait UniqueEmailAddresses {
    fn num_unique_emails(emails: Vec<String>) -> i32;
}

impl UniqueEmailAddresses for Solution {
    fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut unique_emails: Vec<String> = vec![];
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
}
