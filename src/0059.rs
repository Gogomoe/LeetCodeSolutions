use std::iter;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let d1: (i32, i32) = (0, 1);
        let d2: (i32, i32) = (1, 0);
        let d3: (i32, i32) = (0, -1);
        let d4: (i32, i32) = (-1, 0);
        let mut e1 = (0, (n - 1) as usize);
        let mut e2 = ((n - 1) as usize, (n - 1) as usize);
        let mut e3 = ((n - 1) as usize, 0);
        let mut e4 = (1, 0);

        let mut dir = d1;
        let mut pos = (0usize, 0usize);
        let mut num = 1;
        let mut result: Vec<Vec<i32>> = iter::repeat(
            iter::repeat(0).take(n as usize).collect()
        ).take(n as usize).collect();

        loop {
            result[pos.0][pos.1] = num;
            if num == n * n {
                break;
            }

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
            }

            pos = ((pos.0 as i32 + dir.0) as usize, (pos.1 as i32 + dir.1) as usize);
            num += 1;
        }

        result
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::generate_matrix(3)); // [[1,2,3],[8,9,4],[7,6,5]]
    println!("{:?}", Solution::generate_matrix(1)); // [[1]]
}