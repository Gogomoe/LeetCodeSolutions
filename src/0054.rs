impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let d1: (i32, i32) = (0, 1);
        let d2: (i32, i32) = (1, 0);
        let d3: (i32, i32) = (0, -1);
        let d4: (i32, i32) = (-1, 0);
        let mut e1 = (0, cols - 1);
        let mut e2 = (rows - 1, cols - 1);
        let mut e3 = (rows - 1, 0);
        let mut e4 = (1, 0);

        let mut dir = d1;
        let mut pos = (0, 0);
        let mut result = Vec::new();
        result.push(matrix[0][0]);
        loop {
            if (dir == d1 && pos == e1) || (dir == d2 && pos == e2) || (dir == d3 && pos == e3) || (dir == d4 && pos == e4) {
                let next_dir = match dir {
                    _ if dir == d1 => d2,
                    _ if dir == d2 => d3,
                    _ if dir == d3 => d4,
                    _ => d1
                };
                match dir {
                    _ if dir == d1 => { e1 = (e1.0 + 1, e1.1 - 1) }
                    _ if dir == d2 => { e2 = (e2.0 - 1, e2.1 - 1) }
                    _ if dir == d3 => { e3 = (e3.0 - 1, e3.1 + 1) }
                    _ => { e4 = (e4.0 + 1, e4.1 + 1) }
                }
                dir = next_dir;
                if (dir == d1 && pos == e1) || (dir == d2 && pos == e2) || (dir == d3 && pos == e3) || (dir == d4 && pos == e4) {
                    break;
                }
            }

            pos = ((pos.0 as i32 + dir.0) as usize, (pos.1 as i32 + dir.1) as usize);
            result.push(matrix[pos.0 as usize][pos.1 as usize]);
        }

        result
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])); // [1,2,3,6,9,8,7,4,5]
    println!("{:?}", Solution::spiral_order(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]])); // [1,2,3,4,8,12,11,10,9,5,6,7]
}