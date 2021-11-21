use std::ops::Add;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }
        let chars: Vec<char> = s.chars().collect();
        let prefix = get_prefix_len(&chars);
        let mut l = 0;
        let mut r = chars.len() - 1;
        while l < r {
            while l != 0 {
                if chars[l] == chars[r] {
                    break;
                } else {
                    l = prefix[l - 1];
                }
            }
            if chars[l] == chars[r] {
                l += 1;
            }
            r -= 1;
        }

        return if l == r {
            s[l + 1..].chars().rev().collect::<String>().add(&s[l..])
        } else {
            s[l..].chars().rev().collect::<String>().add(&s[l..])
        };
    }
}

// similar to KMP. Is prefix array but not next array.
fn get_prefix_len(chars: &Vec<char>) -> Vec<usize> {
    let mut prefix = Vec::with_capacity(chars.len());
    let mut count = 0;
    prefix.push(0);
    for i in 1..chars.len() {
        while count != 0 {
            if chars[i] == chars[count] {
                break;
            } else {
                count = prefix[count - 1];
            }
        }
        if chars[i] == chars[count] {
            count += 1;
        }
        prefix.push(count);
    }
    return prefix;
}

struct Solution {}

fn main() {
    println!("{}", Solution::shortest_palindrome("aacecaaa".to_string())); // aaacecaaa
    println!("{}", Solution::shortest_palindrome("abcd".to_string())); // dcbabcd
}