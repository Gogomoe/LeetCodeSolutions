use std::iter;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        let m = board.len();
        let n = board[0].len();
        let mut visit: Vec<Vec<bool>> = iter::repeat(
            iter::repeat(false).take(n).collect()
        ).take(m).collect();
        for i in 0..m {
            for j in 0..n {
                let result = dfs(i, j, 0, &chars, &board, &mut visit);
                if result { return true; }
            }
        }
        return false;
    }
}

fn dfs(y: usize, x: usize, pos: usize, chars: &Vec<char>, board: &Vec<Vec<char>>, visit: &mut Vec<Vec<bool>>) -> bool {
    if visit[y][x] { return false; }
    if board[y][x] != chars[pos] { return false; }
    if pos == chars.len() - 1 { return true; }

    visit[y][x] = true;
    if x > 0 {
        let result = dfs(y, x - 1, pos + 1, chars, board, visit);
        if result { return true; };
    }
    if y > 0 {
        let result = dfs(y - 1, x, pos + 1, chars, board, visit);
        if result { return true; };
    }
    if x < board[0].len() - 1 {
        let result = dfs(y, x + 1, pos + 1, chars, board, visit);
        if result { return true; };
    }
    if y < board.len() - 1 {
        let result = dfs(y + 1, x, pos + 1, chars, board, visit);
        if result { return true; };
    }
    visit[y][x] = false;

    return false;
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::exist(
        vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']],
        "ABCCED".to_string(),
    )); // true
    println!("{:?}", Solution::exist(
        vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']],
        "SEE".to_string(),
    )); // true
    println!("{:?}", Solution::exist(
        vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']],
        "ABCB".to_string(),
    )); // false
}