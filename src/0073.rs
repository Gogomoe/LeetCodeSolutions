use std::iter;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut cols: Vec<bool> = iter::repeat(false).take(n).collect();

        for i in 0..m {
            let mut row = false;
            for j in 0..n {
                if matrix[i][j] == 0 {
                    row = true;
                    cols[j] = true;
                }
            }
            if row {
                for j in 0..n {
                    matrix[i][j] = 0;
                }
            }
        }

        for i in 0..n {
            if cols[i] {
                for j in 0..m {
                    matrix[j][i] = 0;
                }
            }
        }
    }
}

struct Solution {}

fn main() {
    let mut vec1 = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    Solution::set_zeroes(&mut vec1);
    println!("{:?}", vec1); // [[1,0,1],[0,0,0],[1,0,1]]

    let mut vec2 = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    Solution::set_zeroes(&mut vec2);
    println!("{:?}", vec2); // [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
}