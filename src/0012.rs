use std::ops::Add;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let ones: Vec<&str> = vec!["I", "X", "C", "M"];
        let fives: Vec<&str> = vec!["V", "L", "D", ""];

        let mut result = Vec::new();
        let mut i = 0;
        while num != 0 {
            let last = num % 10;
            result.push(match last {
                1 | 2 | 3 => { ones[i].repeat(last as usize) }
                4 => { ones[i].to_string() + fives[i] }
                5 => { fives[i].to_string() }
                6 | 7 | 8 => { fives[i].to_string() + ones[i].repeat(last as usize - 5).as_str() }
                9 => { ones[i].to_string() + ones[i + 1] }
                _ => { "".to_string() }
            });

            num /= 10;
            i += 1;
        }

        return result.into_iter().rev().fold(String::new(), |a, b| a.add(b.as_str()));
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::int_to_roman(3724)); // MMMDCCXXIV
}