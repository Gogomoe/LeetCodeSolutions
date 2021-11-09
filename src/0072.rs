use std::cmp::min;
use std::iter;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let w1: Vec<char> = word1.chars().collect();
        let w2: Vec<char> = word2.chars().collect();

        let mut dp: Vec<Vec<i32>> = iter::repeat(
            iter::repeat(0).take(w2.len() + 1).collect()
        ).take(w1.len() + 1).collect();
        for i in 0..=word1.len() {
            dp[i][0] = i as i32;
        }
        for i in 0..=word2.len() {
            dp[0][i] = i as i32;
        }

        for i in 1..=w1.len() {
            for j in 1..=w2.len() {
                let c1 = w1[i - 1];
                let c2 = w2[j - 1];
                let mut result = i32::MAX;
                if c1 == c2 {
                    result = min(result, dp[i - 1][j - 1]);
                } else {
                    result = min(result, dp[i - 1][j - 1] + 1);
                }
                result = min(result, dp[i - 1][j] + 1);
                result = min(result, dp[i][j - 1] + 1);
                dp[i][j] = result;
            }
        }

        return dp[w1.len()][w2.len()];
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::min_distance("horse".to_string(), "ros".to_string())); // 3
    println!("{}", Solution::min_distance("intention".to_string(), "execution".to_string())); // 5
}