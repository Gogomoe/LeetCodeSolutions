use std::iter;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() == 0 {
            return s2 == s3;
        }
        if s2.len() == 0 {
            return s1 == s3;
        }
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let len1 = s1.len();
        let len2 = s2.len();
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let s3: Vec<char> = s3.chars().collect();
        let mut dp: Vec<Vec<bool>> = iter::repeat(
            iter::repeat(false).take(len2 + 1).collect()
        ).take(len1 + 1).collect();

        dp[0][0] = true;
        for i in 0..=len1 {
            for j in 0..=len2 {
                if i != 0 && s3[i + j - 1] == s1[i - 1] {
                    dp[i][j] |= dp[i - 1][j];
                }
                if j != 0 && s3[i + j - 1] == s2[j - 1] {
                    dp[i][j] |= dp[i][j - 1];
                }
            }
        }

        return dp[len1][len2];
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbcbcac".to_string()));
    // true
    println!("{}", Solution::is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbbaccc".to_string()));
    // false
    println!("{}", Solution::is_interleave("".to_string(), "".to_string(), "".to_string()));
    // true
}
