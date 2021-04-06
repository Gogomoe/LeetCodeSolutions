use std::collections::VecDeque;
use std::iter;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut vec: Vec<i32> = iter::repeat(0).take(s.len()).collect();
        let chars: Vec<char> = s.chars().collect();
        for i in (0..s.len()).rev() {
            if chars[i] == '0' {
                continue;
            }
            vec[i] += *vec.get(i + 1).unwrap_or(&1);
            if i + 2 <= s.len() && (chars[i] == '1' || (chars[i] == '2' && chars[i + 1] <= '6')) {
                vec[i] += *vec.get(i + 2).unwrap_or(&1);
            }
        }
        vec[0]
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::num_decodings(String::from("226")));
}