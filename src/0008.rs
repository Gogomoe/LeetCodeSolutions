use std::collections::VecDeque;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut chars = s.trim_start().chars().collect::<VecDeque<char>>();
        let sign = match chars.front() {
            Some(c) => {
                if *c == '+' {
                    chars.pop_front();
                    1
                } else if *c == '-' {
                    chars.pop_front();
                    -1
                } else {
                    1
                }
            }
            None => return 0
        };

        let mut result: u64 = 0;
        while let Some(c) = chars.pop_front() {
            if c >= '0' && c <= '9' {
                result *= 10;
                result += c as u64 - '0' as u64;
                if sign == 1 && result > i32::MAX as u64 {
                    return i32::MAX;
                } else if sign == -1 && result > i32::MAX as u64 + 1 {
                    return i32::MIN;
                }
            } else {
                break;
            }
        }

        (sign as i64 * result as i64) as i32
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::my_atoi("42".to_string()));
    println!("{}", Solution::my_atoi("   -42".to_string()));
    println!("{}", Solution::my_atoi("4193 with words".to_string()));
    println!("{}", Solution::my_atoi("words and 987".to_string()));
    println!("{}", Solution::my_atoi("-91283472332".to_string()));
}