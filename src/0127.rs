use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_set: HashSet<String> = HashSet::new();
        for it in word_list {
            word_set.insert(it);
        }
        if !word_set.contains(&end_word) {
            return 0;
        }

        let mut deque = VecDeque::new();
        let mut visit = HashSet::new();
        for w2 in word_set.iter() {
            let w1 = &begin_word;
            if diff(w1, w2) == 1 {
                deque.push_back((1, w2.clone()));
                visit.insert(w2.clone());
            }
        }

        for it in visit.iter() {
            word_set.remove(it);
        }
        visit.clear();

        let mut level = 0;
        while !deque.is_empty() {
            let (len, word) = deque.pop_front().unwrap();
            if level < len {
                level = len;
                for it in visit.iter() {
                    word_set.remove(it);
                }
                visit.clear();
            }
            if word == end_word {
                return level as i32 + 1;
            } else {
                let mut chars: Vec<char> = word.chars().collect();
                for i in 0..chars.len() {
                    let char = chars[i];
                    for c in 'a'..='z' {
                        chars[i] = c;
                        let next = chars.iter().collect::<String>();
                        if word_set.contains(&next) {
                            visit.insert(next.clone());
                            deque.push_back((level + 1, next.clone()));
                        }
                    }
                    chars[i] = char;
                }
            }
        }

        return 0;
    }
}

fn diff(s1: &String, s2: &String) -> i32 {
    let mut diff = 0;
    let mut c1 = s1.chars();
    let mut c2 = s2.chars();
    while let Some(a) = c1.next() {
        if a != c2.next().unwrap() {
            diff += 1;
        }
    }
    return diff;
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::ladder_length(
        "hit".to_string(), "cog".to_string(),
        vec!["hot".to_string(), "dot".to_string(), "dog".to_string(), "lot".to_string(), "log".to_string(), "cog".to_string()],
    )); // 5
    println!("{:?}", Solution::ladder_length(
        "hit".to_string(), "cog".to_string(),
        vec!["hot".to_string(), "dot".to_string(), "dog".to_string(), "lot".to_string(), "log".to_string()],
    )); // 0
}