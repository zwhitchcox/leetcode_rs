struct Solution;

use std::collections::HashMap;

impl Solution {
    fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        let mut hm: HashMap<String, usize> = HashMap::new();
        for s in cpdomains {
            let (domains, count) = Solution::parse(s);
            for domain in domains {
                let e = hm.entry(domain).or_insert(0);
                *e += count;
            }
        }
        for (domain, count) in hm {
            res.push(format!("{} {}", count, domain));
        }
        res
    }

    fn parse(s: String) -> (Vec<String>, usize) {
        let mut domains: Vec<String> = vec![];
        let mut iter = s.split_whitespace();
        let count: usize = iter.next().unwrap().parse::<usize>().unwrap();
        let domain: String = iter.next().unwrap().parse::<String>().unwrap();
        for (i, c) in domain.chars().enumerate() {
            if c == '.' {
                let subdomain = &domain[i + 1..];
                domains.push(subdomain.to_string());
            }
        }
        domains.push(domain);
        (domains, count)
    }
}

#[test]
fn test() {
    let input: Vec<String> = ["9001 discuss.leetcode.com"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let mut answer: Vec<String> = ["9001 discuss.leetcode.com", "9001 leetcode.com", "9001 com"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let mut output = Solution::subdomain_visits(input);
    answer.sort();
    output.sort();
    assert_eq!(answer, output);

    let input: Vec<String> = [
        "900 google.mail.com",
        "50 yahoo.com",
        "1 intel.mail.com",
        "5 wiki.org",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let mut answer: Vec<String> = [
        "901 mail.com",
        "50 yahoo.com",
        "900 google.mail.com",
        "5 wiki.org",
        "5 org",
        "1 intel.mail.com",
        "951 com",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let mut output = Solution::subdomain_visits(input);
    answer.sort();
    output.sort();
    assert_eq!(answer, output);
}
