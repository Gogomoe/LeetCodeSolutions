use std::collections::VecDeque;

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut chars: VecDeque<char> = s.chars().collect();

        if chars.contains(&'.') {
            if !Solution::resume_decimal_number(&mut chars) {
                return false;
            };
        } else {
            if !Solution::resume_integer(&mut chars) {
                return false;
            };
        }

        if let Some(first) = chars.front() {
            if *first == 'e' || *first == 'E' {
                chars.pop_front();
                if !Solution::resume_integer(&mut chars) {
                    return false;
                };
            }
        }

        return chars.is_empty();
    }

    fn resume_decimal_number(chars: &mut VecDeque<char>) -> bool {
        let mut result = false;
        Solution::resume_plus_or_minus(chars);
        result |= Solution::resume_digits(chars);
        if chars.front().is_none() || *chars.front().unwrap() != '.' {
            return false;
        }
        chars.pop_front();
        result |= Solution::resume_digits(chars);
        return result;
    }

    fn resume_integer(chars: &mut VecDeque<char>) -> bool {
        Solution::resume_plus_or_minus(chars);
        return Solution::resume_digits(chars);
    }

    fn resume_plus_or_minus(chars: &mut VecDeque<char>) -> bool {
        if let Some(first) = chars.front() {
            if *first == '+' || *first == '-' {
                chars.pop_front();
                return true;
            }
        }
        return false;
    }

    fn resume_digits(chars: &mut VecDeque<char>) -> bool {
        let mut result = false;
        while let Some(first) = chars.front() {
            if *first as i32 >= '0' as i32 && *first as i32 <= '9' as i32 {
                chars.pop_front();
                result = true;
            } else {
                break;
            }
        }
        return result;
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::is_number("0".to_string())); // true
    println!("{}", Solution::is_number("e".to_string())); // false
    println!("{}", Solution::is_number(".".to_string())); // false
    println!("{}", Solution::is_number(".1".to_string())); // true
}