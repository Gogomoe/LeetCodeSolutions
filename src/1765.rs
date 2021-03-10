use std::iter;
use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = is_water.len();
        let n = is_water[0].len();

        let mut height: Vec<Vec<i32>> = iter::repeat(
            iter::repeat(-1).take(n).collect()
        ).take(m).collect();

        let mut queue = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    height[i][j] = 0;
                    queue.push_back((i, j));
                }
            }
        }

        while !queue.is_empty() {
            let (i, j) = queue.pop_front().unwrap();
            for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                let (ni, nj) = (i as i32 + di, j as i32 + dj);
                if ni < 0 || ni >= m as i32 || nj < 0 || nj >= n as i32 {
                    continue;
                }
                let (ni, nj) = (ni as usize, nj as usize);
                if height[ni][nj] != -1 { continue; }
                height[ni][nj] = height[i][j] + 1;
                queue.push_back((ni, nj));
            }
        }

        height
    }
}

struct Solution {}

fn main() {}