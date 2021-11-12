use std::cmp::max;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut farest = 0;
        for i in 0..nums.len() {
            if i > farest {
                return false;
            }
            farest = max(farest, i + nums[i] as usize);
            if farest >= nums.len() - 1 {
                return true;
            }
        }
        return false;
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::can_jump(vec![2, 3, 1, 1, 4])); // true
    println!("{}", Solution::can_jump(vec![3, 2, 1, 0, 4])); // false
}