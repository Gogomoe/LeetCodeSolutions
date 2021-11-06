use std::iter;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if s.is_empty() {
            return p.chars().all(|it| it == '*');
        }
        let sc: Vec<char> = s.chars().collect();
        let pc: Vec<char> = p.chars().collect();
        let mut si = 0;
        let mut pi = 0;
        while si < s.len() && pi < p.len() {
            if sc[si] == pc[pi] || pc[pi] == '?' {
                si += 1;
                pi += 1;
            } else {
                break;
            }
        }
        if si == s.len() && pi == p.len() {
            return true;
        }

        let mut sj = s.len() - 1;
        let mut pj = p.len() - 1;
        while sj > si && pj > pi {
            if sc[sj] == pc[pj] || pc[pj] == '?' {
                sj -= 1;
                pj -= 1;
            } else {
                break;
            }
        }

        return Solution::dp_match(s[si..(sj + 1)].to_string(), p[pi..(pj + 1)].to_string());
    }

    pub fn dp_match(s: String, p: String) -> bool {
        let sc: Vec<char> = s.chars().collect();
        let pc: Vec<char> = p.chars().collect();
        let mut dp: Vec<Vec<bool>> = iter::repeat(iter::repeat(false).take(p.len() + 1).collect()).take(s.len() + 1).collect();
        dp[0][0] = true;
        for i in 0..p.len() {
            if pc[i] == '*' {
                dp[0][i + 1] = true;
            } else {
                break;
            }
        }

        for i in 0..s.len() {
            for j in 0..p.len() {
                let schar = sc[i];
                let pchar = pc[j];
                if schar == pchar || pchar == '?' {
                    dp[i + 1][j + 1] = dp[i][j];
                } else if pchar == '*' {
                    dp[i + 1][j + 1] = dp[i + 1][j] || dp[i][j + 1];
                }
            }
        }

        return dp[s.len()][p.len()];
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::is_match("aa".to_string(), "a".to_string())); // false
    println!("{:?}", Solution::is_match("aa".to_string(), "*".to_string())); // true
    println!("{:?}", Solution::is_match("ab".to_string(), "?a".to_string())); // false
    println!("{:?}", Solution::is_match("adceb".to_string(), "*a*b".to_string())); // true
    println!("{:?}", Solution::is_match("acdcb".to_string(), "a*c?b".to_string())); // false
    println!("{:?}", Solution::is_match("aaaaaaaaaaaaab".to_string(), "*aaaaab".to_string())); // true
    println!("{:?}", Solution::is_match("".to_string(), "*****".to_string())); // true
}