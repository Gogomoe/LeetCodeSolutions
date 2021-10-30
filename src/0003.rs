use std::cmp::max;
use std::collections::{HashMap};

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = HashMap::new();
        let mut max_length = 0;
        let mut start_pos = 0;
        for (i, c) in s.chars().enumerate() {
            let i = i as i32;
            if *set.get(&c).unwrap_or(&-1) >= start_pos {
                start_pos = set[&c] + 1;
            }
            set.insert(c, i);
            max_length = max(max_length, i - start_pos + 1);
        }

        max_length
    }
}


struct Solution {}

fn main() {
    println!("{}", Solution::length_of_longest_substring("dvdf".to_string())); // 3
    println!("{}", Solution::length_of_longest_substring("pwwkew".to_string())); // 3
    println!("{}", Solution::length_of_longest_substring("cdd".to_string())); // 2
}