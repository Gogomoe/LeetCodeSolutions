use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut dict = HashMap::new();
        for str in words.clone() {
            *dict.entry(str).or_insert(0 as i32) += 1;
        }

        let mut result = Vec::new();
        let word_len = words.first().unwrap().len();
        let word_cnt = words.len();
        let str_len = word_len * word_cnt;
        for i in 0..word_len {
            if i + str_len > s.len() {
                break;
            }
            let mut values = dict.clone();
            for j in (i..i + str_len).step_by(word_len) {
                let word = &s[j..j + word_len];
                *values.entry(word.to_string()).or_insert(0) -= 1;
                if *values.get(word).unwrap() == 0 {
                    values.remove(word);
                }
            }
            if values.is_empty() {
                result.push(i as i32);
            }
            for j in ((i + word_len)..=s.len() - str_len).step_by(word_len) {
                let remove = &s[j - word_len..j];
                let add = &s[j - word_len + str_len..j + str_len];

                *values.entry(remove.to_string()).or_insert(0) += 1;
                if *values.get(remove).unwrap() == 0 {
                    values.remove(remove);
                }
                *values.entry(add.to_string()).or_insert(0) -= 1;
                if *values.get(add).unwrap() == 0 {
                    values.remove(add);
                }

                if values.is_empty() {
                    result.push(j as i32);
                }
            }
        }

        result
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::find_substring(
        "barfoothefoobarman".to_string(),
        vec!["foo".to_string(), "bar".to_string()],
    ));
    println!("{:?}", Solution::find_substring(
        "wordgoodgoodgoodbestword".to_string(),
        vec!["word".to_string(), "good".to_string(), "best".to_string(), "word".to_string()],
    ));
    println!("{:?}", Solution::find_substring(
        "barfoofoobarthefoobarman".to_string(),
        vec!["foo".to_string(), "bar".to_string(), "the".to_string()],
    ));
    println!("{:?}", Solution::find_substring(
        "a".to_string(),
        vec!["a".to_string()],
    ));
    println!("{:?}", Solution::find_substring(
        "aaaaaaaaaaaaaa".to_string(),
        vec!["aa".to_string(), "aa".to_string()],
    ));
}