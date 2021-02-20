impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        if rows == 0 { return 0; }
        let columns = grid[0].len();
        if columns == 0 { return 0; }

        let mut row_state: Vec<i32> = Vec::new();
        let mut col_state: Vec<i32> = Vec::new();
        for _ in 0..rows { row_state.push(0) }
        for _ in 0..columns { col_state.push(0) }

        let mut result = 0;

        for i in 0..rows {
            for j in 0..columns {
                if grid[i][j] == 0 { continue; }
                row_state[i] += 1;
                col_state[j] += 1;
                result += 1;
            }
        }

        for i in 0..rows {
            for j in 0..columns {
                if grid[i][j] == 0 { continue; }
                if row_state[i] == 1 && col_state[j] == 1 { result -= 1; }
            }
        }

        result
    }
}

struct Solution {}

fn main() {}