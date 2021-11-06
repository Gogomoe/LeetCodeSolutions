use std::collections::HashSet;
use std::iter;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut col = HashSet::new();
        let mut diag1 = HashSet::new();
        let mut diag2 = HashSet::new();
        let mut result = Vec::new();
        let mut pos: Vec<Vec<char>> = iter::repeat(iter::repeat('.').take(n as usize).collect()).take(n as usize).collect();
        Solution::dfs(0, &mut pos, &mut col, &mut diag1, &mut diag2, &mut result, n);
        return result;
    }

    fn dfs(level: i32, pos: &mut Vec<Vec<char>>, col: &mut HashSet<i32>, diag1: &mut HashSet<i32>, diag2: &mut HashSet<i32>, result: &mut Vec<Vec<String>>, n: i32) {
        if level == n {
            result.push(pos.iter().map(|it| it.iter().collect()).collect());
            return;
        }
        for j in 0..n {
            let d1 = level - j;
            let d2 = level + j;
            if col.contains(&j) || diag1.contains(&d1) || diag2.contains(&d2) {
                continue;
            }
            col.insert(j);
            diag1.insert(d1);
            diag2.insert(d2);
            pos[level as usize][j as usize] = 'Q';
            Solution::dfs(level + 1, pos, col, diag1, diag2, result, n);
            pos[level as usize][j as usize] = '.';
            col.remove(&j);
            diag1.remove(&d1);
            diag2.remove(&d2);
        }
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::solve_n_queens(4)); // [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
}