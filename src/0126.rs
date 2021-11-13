use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        let mut word_set: HashSet<String> = HashSet::new();
        for it in word_list {
            word_set.insert(it);
        }
        if !word_set.contains(&end_word) {
            return Vec::new();
        }
        let mut result = Vec::new();

        let mut deque = VecDeque::new();
        let mut visit = HashSet::new();
        for w2 in word_set.iter() {
            let w1 = &begin_word;
            if diff(w1, w2) == 1 {
                deque.push_back((vec![w1.clone()], w2.clone()));
                visit.insert(w2.clone());
            }
        }

        for it in visit.iter() {
            word_set.remove(it);
        }
        visit.clear();

        let mut res_len = usize::MAX;
        let mut level = 0;
        while !deque.is_empty() {
            let (mut list, word) = deque.pop_front().unwrap();
            if level < list.len() {
                level = list.len();
                for it in visit.iter() {
                    word_set.remove(it);
                }
                visit.clear();
            }
            if level > res_len {
                break;
            }
            if word == end_word && (res_len == usize::MAX || res_len == level) {
                res_len = level;
                list.push(end_word.clone());
                result.push(list);
            } else {
                let mut chars: Vec<char> = word.chars().collect();
                for i in 0..chars.len() {
                    let char = chars[i];
                    for c in 'a'..='z' {
                        chars[i] = c;
                        let next = chars.iter().collect::<String>();
                        if word_set.contains(&next) {
                            visit.insert(next.clone());
                            let mut path = list.clone();
                            path.push(word.clone());
                            deque.push_back((path, next.clone()));
                        }
                    }
                    chars[i] = char;
                }
            }
        }

        result
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
    println!("{:?}", Solution::find_ladders(
        "hit".to_string(), "cog".to_string(),
        vec!["hot".to_string(), "dot".to_string(), "dog".to_string(), "lot".to_string(), "log".to_string(), "cog".to_string()],
    )); // [["hit","hot","dot","dog","cog"],["hit","hot","lot","log","cog"]]
    println!("{:?}", Solution::find_ladders(
        "hit".to_string(), "cog".to_string(),
        vec!["hot".to_string(), "dot".to_string(), "dog".to_string(), "lot".to_string(), "log".to_string()],
    )); // []
}