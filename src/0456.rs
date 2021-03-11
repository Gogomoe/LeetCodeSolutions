use std::cmp::min;
use std::collections::VecDeque;
use std::iter;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut min_before = Vec::new();
        let mut minimal = nums[0];
        for num in nums.iter() {
            minimal = min(minimal, *num);
            min_before.push(minimal);
        }

        let mut stack = VecDeque::new();

        for i in (0..nums.len()).rev() {
            let it = nums[i];
            while !stack.is_empty() && nums[stack[0]] < it {
                let j = stack.pop_front().unwrap();
                if nums[j] > min_before[i] {
                    return true;
                }
            }
            stack.push_front(i);
        }

        false
    }
}

struct Solution {}

fn main() {
    let res = Solution::find132pattern(vec![3, 1, 4, 2]);
    println!("{}", res);
}