/*
 * [0929] unique-email-addresses
 */

pub struct Solution {}

// solution impl starts here

use std::collections::HashSet;
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set = HashSet::new();
        for email in emails.iter() {
            set.insert(Solution::helper(email.to_string()));
        }
        set.len() as i32
    }

    pub fn helper(email: String) -> String {
        // 分离地址和域名
        let mut parts = email.splitn(2, "@");
        let addr = parts.next().unwrap();
        let domain = parts.next().unwrap();

        // 分离加号
        let mut addr_parts = addr.splitn(2, "+");
        let addr = addr_parts.next().unwrap();

        // 去除点
        let addr = &addr.replace(".", "");
        addr.to_owned() + "@" + domain
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper() {
        assert_eq!(
            Solution::helper("m.y+name@email.com".to_string()),
            "my@email.com"
        );
    }

    #[test]
    fn test_case0() {
        assert_eq!(
            Solution::num_unique_emails(vec_string![
                "test.email+alex@leetcode.com",
                "test.e.mail+bob.cathy@leetcode.com",
                "testemail+david@lee.tcode.com"
            ]),
            2
        );
    }
}

// solution tests ends here
