use std::cmp::min;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m {
            for j in 0..n {
                let mut val = i32::MAX;
                if i != 0 {
                    val = min(val, grid[i - 1][j]);
                }
                if j != 0 {
                    val = min(val, grid[i][j - 1]);
                }
                if val != i32::MAX {
                    grid[i][j] += val;
                }
            }
        }
        return grid[m - 1][n - 1];
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]])); // 7
    println!("{}", Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]])); // 12
}
