use std::collections::VecDeque;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let m = grid.len();
        let n = grid.get(0).map_or_else(|| { 0 }, Vec::len);

        let mut queue = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    count += 1;
                    queue.push_back((i, j));
                    grid[i][j] = '2';
                    while !queue.is_empty() {
                        let (y, x) = queue.pop_front().unwrap();
                        if y > 0 && grid[y - 1][x] == '1' {
                            grid[y - 1][x] = '2';
                            queue.push_back((y - 1, x));
                        }
                        if x > 0 && grid[y][x - 1] == '1' {
                            grid[y][x - 1] = '2';
                            queue.push_back((y, x - 1));
                        }
                        if y < m - 1 && grid[y + 1][x] == '1' {
                            grid[y + 1][x] = '2';
                            queue.push_back((y + 1, x));
                        }
                        if x < n - 1 && grid[y][x + 1] == '1' {
                            grid[y][x + 1] = '2';
                            queue.push_back((y, x + 1));
                        }
                    }
                }
            }
        }

        count
    }
}

struct Solution {}

fn main() {}