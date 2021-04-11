use std::cmp::max;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut max_area = 0;
        for i in 0..heights.len() {
            let it = heights[i];
            while !stack.is_empty() && heights[*stack.last().unwrap()] > it {
                let j = stack.pop().unwrap();
                let h = heights[j];
                let area = match stack.last() {
                    None => {
                        h * (i as i32 - 1 - -1)
                    }
                    Some(it) => {
                        h * (i as i32 - 1 - *it as i32)
                    }
                };
                max_area = max(max_area, area);
            }
            stack.push(i)
        }

        while !stack.is_empty() {
            let j = stack.pop().unwrap();
            let h = heights[j];
            let area = match stack.last() {
                None => {
                    h * (heights.len() as i32 - 1 - -1)
                }
                Some(it) => {
                    h * (heights.len() as i32 - 1 - *it as i32)
                }
            };
            max_area = max(max_area, area);
        }

        max_area
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::largest_rectangle_area(vec![2, 4]))
}