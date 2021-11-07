use std::collections::HashSet;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut result = 0;
        let mut col = HashSet::new();
        let mut diag1 = HashSet::new();
        let mut diag2 = HashSet::new();
        Solution::dfs(0, &mut result, &mut col, &mut diag1, &mut diag2, n);
        return result;
    }

    fn dfs(i: i32, result: &mut i32, col: &mut HashSet<i32>, diag1: &mut HashSet<i32>, diag2: &mut HashSet<i32>, n: i32) {
        if i == n {
            *result += 1;
            return;
        }

        for j in 0..n {
            if col.contains(&j) || diag1.contains(&(i + j)) || diag2.contains(&(i - j)) {
                continue;
            }
            col.insert(j);
            diag1.insert(i + j);
            diag2.insert(i - j);
            Solution::dfs(i + 1, result, col, diag1, diag2, n);
            col.remove(&j);
            diag1.remove(&(i + j));
            diag2.remove(&(i - j));
        }
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::total_n_queens(4)); // 2
    println!("{:?}", Solution::total_n_queens(1)); // 1
}