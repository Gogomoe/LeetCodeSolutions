use std::cmp::{max, min};
use std::iter;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();
        let mut arr: Vec<Vec<i32>> = iter::repeat(
            iter::repeat(0).take(n).collect()
        ).take(m).collect();
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let val = dungeon[i][j];
                if i == m - 1 && j == n - 1 {
                    arr[i][j] = 1 - val;
                } else if i == m - 1 {
                    arr[i][j] = max(arr[i][j + 1], 1) - val;
                } else if j == n - 1 {
                    arr[i][j] = max(arr[i + 1][j], 1) - val;
                } else {
                    arr[i][j] = min(max(arr[i][j + 1], 1), max(arr[i + 1][j], 1)) - val;
                }
                arr[i][j] = max(arr[i][j], 1);
            }
        }
        return arr[0][0];
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::calculate_minimum_hp(vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]])); // 7
}