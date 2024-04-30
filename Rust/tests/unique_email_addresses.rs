#[cfg(test)]
mod tests {
    use leetcode::unique_email_addresses::*;

    #[test]
    fn test_unique_emails_address_case1() {
        let emails = vec![
            "test.email+alex@leetcode.com".to_string(),
            "test.e.mail+bob.cathy@leetcode.com".to_string(),
            "testemail+david@lee.tcode.com".to_string(),
        ];
        let want = 2;

        let result = num_unique_emails(emails.clone());
        let result2 = num_unique_emails2(emails);

        assert_eq!(result, want);
        assert_eq!(result2, want);
    }

    #[test]
    fn test_unique_emails_address_case2() {
        let emails = vec![
            "a@leetcode.com".to_string(),
            "b@leetcode.com".to_string(),
            "c@leetcode.com".to_string(),
        ];
        let want = 3;

        let result = num_unique_emails(emails.clone());
        let result2 = num_unique_emails2(emails);

        assert_eq!(result, want);
        assert_eq!(result2, want);
    }
}
