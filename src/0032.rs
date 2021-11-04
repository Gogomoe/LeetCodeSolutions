use std::cmp::max;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut result = 0;

        let mut start = 0;
        let mut stack = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack += 1;
            } else {
                stack -= 1;
            }
            if stack == 0 {
                result = max(result, i - start + 1)
            } else if stack < 0 {
                stack = 0;
                start = i + 1;
            }
        }

        start = 0;
        stack = 0;
        for (i, c) in s.chars().rev().enumerate() {
            if c == ')' {
                stack += 1;
            } else {
                stack -= 1;
            }
            if stack == 0 {
                result = max(result, i - start + 1)
            } else if stack < 0 {
                stack = 0;
                start = i + 1;
            }
        }

        result as i32
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::longest_valid_parentheses("(()".to_string())); // 2
    println!("{:?}", Solution::longest_valid_parentheses(")()())".to_string())); // 4
    println!("{:?}", Solution::longest_valid_parentheses("".to_string())); // 0
}