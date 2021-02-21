use std::cmp::max;

fn find_most_char(counts: &[u32; 26]) -> (u8, u8) {
    let mut first = 0;
    let mut second = 1;
    if counts[1] > counts[0] {
        first = 1;
        second = 0;
    }
    for i in 2..26 {
        if counts[i] > counts[first] {
            second = first;
            first = i;
        } else if counts[i] > counts[second] {
            second = i;
        }
    }
    (first as u8, second as u8)
}

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        if s.len() <= 1 { return s; }

        let chars: Vec<char> = s.chars().collect();
        let mut counts: [u32; 26] = [0; 26];
        let mut max_count = 0;

        for char in chars.iter() {
            counts[(*char as u32 - 'a' as u32) as usize] += 1;
            max_count = max(max_count, counts[(*char as u32 - 'a' as u32) as usize]);
        }

        let mut result = String::new();
        if max_count * 2 - 1 > s.len() as u32 {
            return result;
        }

        let (a, b) = find_most_char(&counts);
        result.push(('a' as u8 + a) as char);
        result.push(('a' as u8 + b) as char);
        counts[a as usize] -= 1;
        counts[b as usize] -= 1;

        let mut remain = s.len() - 2;
        while remain > 1 {
            let (a, b) = find_most_char(&counts);
            if a != (result.chars().last().unwrap() as u8 - 'a' as u8) {
                result.push(('a' as u8 + a) as char);
                result.push(('a' as u8 + b) as char);
            } else {
                result.push(('a' as u8 + b) as char);
                result.push(('a' as u8 + a) as char);
            }
            counts[a as usize] -= 1;
            counts[b as usize] -= 1;
            remain -= 2;
        }

        for i in 0..26 {
            if counts[i] != 0 {
                result.push(('a' as u8 + i as u8) as char);
            }
        }

        result
    }
}

struct Solution {}

fn main() {
    let result = Solution::reorganize_string(String::from("aab"));
    println!("{}", result);
}