impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        let sc: Vec<char> = s.chars().collect();
        let pc: Vec<char> = p.chars().collect();
        dp[0][0] = true;
        for i in 1..p.len() + 1 {
            if pc[i - 1] == '*' {
                dp[0][i] = dp[0][i - 2];
            }
        }
        for i in 1..s.len() + 1 {
            for j in 1..p.len() + 1 {
                let char = sc[i - 1];
                let pchar = pc[j - 1];
                if pchar != '.' && pchar != '*' && char == pchar {
                    dp[i][j] = dp[i - 1][j - 1]
                } else if pchar == '.' {
                    dp[i][j] = dp[i - 1][j - 1]
                } else if pchar == '*' {
                    let prev_pchar = pc[j - 2];
                    dp[i][j] = dp[i][j - 2] || if prev_pchar == '.' {
                        dp[i - 1][j]
                    } else {
                        char == prev_pchar && dp[i - 1][j]
                    }
                }
            }
        }

        return dp[s.len()][p.len()];
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::is_match("aa".to_string(), "a".to_string()));
    println!("{}", Solution::is_match("aa".to_string(), "a*".to_string()));
    println!("{}", Solution::is_match("ab".to_string(), ".*".to_string()));
    println!("{}", Solution::is_match("aab".to_string(), "c*a*b".to_string()));
    println!("{}", Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()));
}