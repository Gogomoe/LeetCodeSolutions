use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        for i in 0..points.len() {
            let mut map = HashMap::new();
            let mut same_y = 0;
            let mut same_x = 0;
            for j in i + 1..points.len() {
                let dy = points[j][1] - points[i][1];
                let dx = points[j][0] - points[i][0];
                if dy == 0 {
                    same_x += 1;
                    continue;
                }
                if dx == 0 {
                    same_y += 1;
                    continue;
                }
                let g = gcd(dy, dx);
                let dy = dy / g;
                let dx = dx / g;
                *map.entry((dy, dx)).or_insert(0) += 1;
            }
            for it in map.values() {
                result = max(result, *it);
            }
            result = max(result, same_x);
            result = max(result, same_y);
        }
        return result + 1;
    }
}

fn gcd(y: i32, x: i32) -> i32 {
    if x == 0 { return y; }
    return gcd(x, y % x);
}

struct Solution {}

fn main() {
    println!("{}", Solution::max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]])); // 3
    println!("{}", Solution::max_points(vec![vec![1, 1], vec![3, 2], vec![5, 3], vec![4, 1], vec![2, 3], vec![1, 4]])); // 4
}