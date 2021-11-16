impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut l = 0;
        let mut r = m - 1;
        while l < r {
            let mid = (l + r + 1) / 2;
            if matrix[mid][0] <= target {
                l = mid;
            } else {
                r = mid - 1;
            }
        }

        let row = l;
        if matrix[row][0] > target {
            return false;
        }

        let mut l = 0;
        let mut r = n - 1;
        while l < r {
            let mid = (l + r + 1) / 2;
            if matrix[row][mid] <= target {
                l = mid;
            } else {
                r = mid - 1;
            }
        }

        return matrix[row][l] == target;
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 3)); // true
    println!("{}", Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 13)); // false
}