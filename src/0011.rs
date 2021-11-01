use std::cmp::{max, min};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut result = (r - l) * min(height[r] as usize, height[l] as usize);
        while l != r {
            if height[r] < height[l] {
                r -= 1;
            } else {
                l += 1;
            }
            result = max(result, (r - l) * min(height[r] as usize, height[l] as usize));
        }
        result as i32
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    println!("{}", Solution::max_area(vec![1, 1]));
    println!("{}", Solution::max_area(vec![4, 3, 2, 1, 4]));
    println!("{}", Solution::max_area(vec![1, 2, 1]));
}