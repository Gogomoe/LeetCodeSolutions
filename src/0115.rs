use std::iter;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut dp: Vec<Vec<i32>> = iter::repeat(
            iter::repeat(-1).take(t.len()).collect()
        ).take(s.len()).collect();
        return helper(0, 0, &mut dp, &s.chars().collect(), &t.chars().collect());
    }
}

fn helper(ps: usize, pt: usize, dp: &mut Vec<Vec<i32>>, s: &Vec<char>, t: &Vec<char>) -> i32 {
    if pt == t.len() {
        return 1;
    }
    if ps == s.len() {
        return 0;
    }
    if dp[ps][pt] != -1 {
        return dp[ps][pt];
    }
    let mut result = 0;
    for i in ps..s.len() {
        if s[i] == t[pt] {
            result += helper(i + 1, pt + 1, dp, s, t);
        }
    }
    dp[ps][pt] = result;
    return result;
}

struct Solution {}

fn main() {
    println!("{}", Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string())); // 3
    println!("{}", Solution::num_distinct("babgbag".to_string(), "bag".to_string())); // 5
}