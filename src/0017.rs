use std::ops::Add;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() { return Vec::new(); }
        let maps = vec![
            vec!["a", "b", "c"],
            vec!["d", "e", "f"],
            vec!["g", "h", "i"],
            vec!["j", "k", "l"],
            vec!["m", "n", "o"],
            vec!["p", "q", "r", "s"],
            vec!["t", "u", "v"],
            vec!["w", "x", "y", "z"],
        ];
        return digits.chars()
            .map(|it| it as i32 - '2' as i32)
            .map(|it| &maps[it as usize])
            .fold(vec![String::new()], |acc, it| {
                let mut result = Vec::new();
                for letter in it.iter() {
                    for pre in acc.iter() {
                        result.push(pre.clone().add(*letter))
                    }
                }
                result
            });
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::letter_combinations("23".to_string()));
    println!("{:?}", Solution::letter_combinations("".to_string()));
    println!("{:?}", Solution::letter_combinations("2".to_string()));
}