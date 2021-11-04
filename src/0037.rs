use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Solution::dfs(board, 0);
    }

    fn dfs(board: &mut Vec<Vec<char>>, pos: usize) -> bool {
        if pos == 81 {
            return true;
        }
        let i = pos / 9;
        let j = pos % 9;
        if board[i][j] != '.' {
            return Solution::dfs(board, pos + 1);
        }

        let mut candidants = HashSet::<char>::from_iter(vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']);
        for k in 0..9 {
            candidants.remove(&board[i][k]);
            candidants.remove(&board[k][j]);
            candidants.remove(&board[i / 3 * 3 + k / 3][j / 3 * 3 + k % 3]);
        }

        for c in candidants {
            board[i][j] = c;
            if Solution::dfs(board, pos + 1) {
                return true;
            }
            board[i][j] = '.'
        }

        return false;
    }
}

struct Solution {}

fn main() {
    let mut vec = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    Solution::solve_sudoku(&mut vec);
    println!("{:?}", vec);
}