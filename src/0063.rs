use std::iter;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut dp: Vec<Vec<i32>> = iter::repeat(
            iter::repeat(0).take(n).collect()
        ).take(m).collect();

        dp[0][0] = if obstacle_grid[0][0] == 1 { return 0; } else { 1 };
        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 0 {
                    dp[i][j] += if i == 0 { 0 } else { dp[i - 1][j] };
                    dp[i][j] += if j == 0 { 0 } else { dp[i][j - 1] };
                }
            }
        }
        return dp[m - 1][n - 1];
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]])); // 2
    println!("{}", Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]])); // 1
}
