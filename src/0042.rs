use std::cmp::min;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut stack = Vec::new();

        for i in 0..height.len() {
            while !stack.is_empty() && height[i] > height[*stack.last().unwrap()] {
                let pop = stack.pop().unwrap();
                if let Some(front) = stack.last() {
                    result += (min(height[i], height[*front]) - height[pop]) * (i - *front - 1) as i32;
                }
            }
            stack.push(i);
        }

        return result;
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])); // 6
}