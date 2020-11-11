pub struct Solution {}

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut count_map:HashMap<String, i32> = HashMap::new();
        for domain in cpdomains.iter(){

            let split = domain.split_whitespace().collect::<Vec<&str>>();

            let count:i32 = split[0].parse().unwrap();
            let domain_item: &str = split[1];
            let sub_vec:Vec<&str> = domain_item.split(".").collect::<Vec<&str>>();
            let length:usize = sub_vec.len();
            let mut start_index:usize = length;
            while start_index > 0 {
                start_index -= 1;
                let sub_domain:String = sub_vec[start_index..length].join(".");
                match count_map.get(&sub_domain) {
                    Some(cnt) => {
                        count_map.insert(sub_domain, *cnt+count);
                    },
                    None => {
                        count_map.insert(sub_domain, count);
                    }
                }
            }
        }

        let mut result:Vec<String> = vec![];
        for (domain, cnt) in count_map.iter(){
            let record:String = cnt.to_string() + " " + domain;
            result.push(record);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        domains: Vec<String>,
        result: Vec<String>,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                domains: vec!["9001 discuss.leetcode.com".to_string()],
                result: vec!["9001 discuss.leetcode.com".to_string(), "9001 leetcode.com".to_string(), "9001 com".to_string()],
            },
            Case {
                domains: vec!["900 google.mail.com".to_string(), "50 yahoo.com".to_string(), "1 intel.mail.com".to_string(), "5 wiki.org".to_string()],
                result: vec!["901 mail.com".to_string(), "50 yahoo.com".to_string(), "900 google.mail.com".to_string(), "5 wiki.org".to_string(), "5 org".to_string(), "1 intel.mail.com".to_string(), "951 com".to_string()],
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::subdomain_visits(case_item.domains.clone());
            assert_eq!(result.len(), case_item.result.len());
            for item in result.iter(){
                assert_eq!(true, case_item.result.contains(item));
            }
        }
    }
}