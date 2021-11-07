use std::collections::HashSet;
use std::iter;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<char>> = iter::repeat(HashSet::new()).take(9).collect();
        let mut cols: Vec<HashSet<char>> = iter::repeat(HashSet::new()).take(9).collect();
        let mut squares: Vec<HashSet<char>> = iter::repeat(HashSet::new()).take(9).collect();

        for i in 0..9 {
            for j in 0..9 {
                let c = board[i][j];
                if c == '.' { continue; }
                if !rows[i].insert(c) || !cols[j].insert(c) || !squares[i / 3 * 3 + j / 3].insert(c) {
                    return false;
                }
            }
        }
        true
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::is_valid_sudoku(vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ])); // true
    println!("{}", Solution::is_valid_sudoku(vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ])); // false
}