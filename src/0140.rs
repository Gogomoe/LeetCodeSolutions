use std::iter;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        return dfs(0, &s, &word_dict, &mut iter::repeat(None).take(s.len()).collect());
    }
}

fn dfs(index: usize, s: &String, word_dict: &Vec<String>, cache: &mut Vec<Option<Vec<String>>>) -> Vec<String> {
    if cache[index].is_some() {
        return cache[index].as_ref().unwrap().clone();
    }
    let mut result = Vec::new();
    for word in word_dict {
        let new_len = index + word.len();
        if new_len <= s.len() && s[index..new_len] == word[0..] {
            if new_len != s.len() {
                for sentence in dfs(new_len, s, word_dict, cache) {
                    result.push(format!("{} {}", word, sentence));
                }
            } else {
                result.push(word.clone());
            }
        }
    }
    cache[index] = Some(result.clone());
    return result;
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::word_break(
        "catsanddog".to_string(),
        vec!["cat".to_string(), "cats".to_string(), "and".to_string(), "sand".to_string(), "dog".to_string()],
    )); // ["cats and dog","cat sand dog"]
    println!("{:?}", Solution::word_break(
        "pineapplepenapple".to_string(),
        vec!["apple".to_string(), "pen".to_string(), "applepen".to_string(), "pine".to_string(), "pineapple".to_string()],
    )); // ["pine apple pen apple","pineapple pen apple","pine applepen apple"]
    println!("{:?}", Solution::word_break(
        "catsandog".to_string(),
        vec!["cat".to_string(), "cats".to_string(), "and".to_string(), "sand".to_string(), "dog".to_string()],
    )); // []
}