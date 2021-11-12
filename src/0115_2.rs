use std::iter;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut dp: Vec<Vec<i32>> = iter::repeat(
            iter::repeat(0).take(s.len()).collect()
        ).take(t.len()).collect();
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        dp[0][0] = if s[s.len() - 1] == t[t.len() - 1] { 1 } else { 0 };
        for i in 1..s.len() {
            if s[s.len() - i - 1] == t[t.len() - 1] {
                dp[0][i] = dp[0][i - 1] + 1
            } else {
                dp[0][i] = dp[0][i - 1]
            }
        }

        for j in 1..t.len() {
            for i in 1..s.len() {
                dp[j][i] += dp[j][i - 1];
                if s[s.len() - i - 1] == t[t.len() - j - 1] {
                    dp[j][i] += dp[j - 1][i - 1];
                }
            }
        }

        return dp[t.len() - 1][s.len() - 1];
    }
}


struct Solution {}

fn main() {
    println!("{}", Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string())); // 3
    println!("{}", Solution::num_distinct("babgbag".to_string(), "bag".to_string())); // 5
}