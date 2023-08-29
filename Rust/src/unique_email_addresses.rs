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

#[test]
fn test_unique_emails_address() {
    struct Tt {
        emails: Vec<String>,
        want: i32,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            emails: vec![
                "test.email+alex@leetcode.com".to_string(),
                "test.e.mail+bob.cathy@leetcode.com".to_string(),
                "testemail+david@lee.tcode.com".to_string(),
            ],
            want: 2,
        },
        Tt {
            emails: vec!["a@leetcode.com".to_string(), "b@leetcode.com".to_string(), "c@leetcode.com".to_string()],
            want: 3,
        },
    ];

    for t in cases.into_iter() {
        let result = Solution::num_unique_emails(t.emails);
        assert_eq!(result, t.want);
    }
}
