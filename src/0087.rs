use std::iter;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let len = s1.len();
        let mut dp: Vec<Vec<Vec<bool>>> = iter::repeat(
            iter::repeat(
                iter::repeat(false).take(len).collect()
            ).take(len).collect()
        ).take(len + 1).collect();

        for i in 0..len {
            for j in 0..len {
                if s1[i] == s2[j] {
                    dp[1][i][j] = true
                }
            }
        }

        for k in 2..=len {
            for i in 0..=(len - k) {
                for j in 0..=(len - k) {
                    for q in 1..k {
                        dp[k][i][j] |= (dp[q][i][j] && dp[k - q][i + q][j + q]) || (dp[q][i][j + k - q] && dp[k - q][i + q][j]);
                        if dp[k][i][j] {
                            break;
                        }
                    }
                }
            }
        }

        return dp[len][0][0];
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::is_scramble("great".to_string(), "rgeat".to_string())); // true
    println!("{}", Solution::is_scramble("abcde".to_string(), "caebd".to_string())); // false
    println!("{}", Solution::is_scramble("a".to_string(), "a".to_string())); // true
}