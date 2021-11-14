use std::cmp::min;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();

        let mut dp = Vec::new();
        for i in 0..s.len() {
            dp.push(i as i32);
        }

        for i in 0..s.len() {
            let mut l = i;
            let mut r = i;
            while l > 0 && r < s.len() - 1 && s[l] == s[r] {
                dp[r] = min(dp[r], dp[l - 1] + 1);
                l -= 1;
                r += 1;
            }
            if s[l] == s[r] {
                dp[r] = min(dp[r], if l == 0 { 0 } else { dp[l - 1] + 1 });
            }

            if i == s.len() - 1 {
                break;
            }
            l = i;
            r = i + 1;
            while l > 0 && r < s.len() - 1 && s[l] == s[r] {
                dp[r] = min(dp[r], dp[l - 1] + 1);
                l -= 1;
                r += 1;
            }
            if s[l] == s[r] {
                dp[r] = min(dp[r], if l == 0 { 0 } else { dp[l - 1] + 1 });
            }
        }

        return dp[s.len() - 1];
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::min_cut("aab".to_string())); // 1
    println!("{}", Solution::min_cut("a".to_string())); // 0
    println!("{}", Solution::min_cut("ab".to_string())); // 1
}