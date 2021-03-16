impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..(n / 2) {
            for j in 0..(n + 1) / 2 {
                let a = matrix[i][j];
                let b = matrix[j][n - i - 1];
                let c = matrix[n - i - 1][n - j - 1];
                let d = matrix[n - j - 1][i];

                matrix[i][j] = d;
                matrix[j][n - i - 1] = a;
                matrix[n - i - 1][n - j - 1] = b;
                matrix[n - j - 1][i] = c;
            }
        }
    }
}

struct Solution {}

fn main() {}