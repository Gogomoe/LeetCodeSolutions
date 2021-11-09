use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut target = HashMap::new();
        for char in t.chars() {
            *target.entry(char).or_insert(0) += 1;
        }

        let mut start = 0;
        let mut len = 0;
        let mut satify = 0;

        let mut min_start = 0;
        let mut min_len = usize::MAX;
        loop {
            if satify < target.keys().len() {
                if start + len == chars.len() {
                    break;
                }
                let char = chars[start + len];
                len += 1;
                if let Some(it) = target.get_mut(&char) {
                    *it -= 1;
                    if *it == 0 {
                        satify += 1;
                    }
                }
            } else {
                if len < min_len {
                    min_start = start;
                    min_len = len;
                }

                let char = chars[start];
                start += 1;
                len -= 1;
                if let Some(it) = target.get_mut(&char) {
                    *it += 1;
                    if *it == 1 {
                        satify -= 1;
                    }
                }
            }
        }

        return if min_len == usize::MAX {
            String::new()
        } else {
            s[min_start..min_start + min_len].to_string()
        };
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string())); // BANC
    println!("{}", Solution::min_window("a".to_string(), "a".to_string())); // a
    println!("{}", Solution::min_window("a".to_string(), "aa".to_string())); // a
}