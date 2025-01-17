struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    fn index_pairs(text: String, words: Vec<String>) -> Vec<Vec<i32>> {
        let n = text.len();
        let mut res: Vec<Vec<i32>> = vec![];
        let hs: HashSet<String> = HashSet::from_iter(words);
        for i in 0..n {
            for j in i..n {
                if hs.contains(&text[i..=j]) {
                    res.push(vec![i as i32, j as i32]);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let text = "thestoryofleetcodeandme".to_string();
    let words: Vec<String> = ["story", "fleet", "leetcode"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let res: Vec<Vec<i32>> = [[3, 7], [9, 13], [10, 17]]
        .iter()
        .map(|v| v.to_vec())
        .collect();
    assert_eq!(Solution::index_pairs(text, words), res);
    let text = "ababa".to_string();
    let words: Vec<String> = ["aba", "ab"].iter().map(|s| s.to_string()).collect();
    let res: Vec<Vec<i32>> = [[0, 1], [0, 2], [2, 3], [2, 4]]
        .iter()
        .map(|v| v.to_vec())
        .collect();
    assert_eq!(Solution::index_pairs(text, words), res);
}
