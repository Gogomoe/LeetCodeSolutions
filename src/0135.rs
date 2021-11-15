use std::cmp::max;
use std::iter;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut left: Vec<i32> = iter::repeat(1).take(n).collect();
        let mut right: Vec<i32> = iter::repeat(1).take(n).collect();
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                left[i] = left[i - 1] + 1;
            }
        }
        for i in (0..(n - 1)).rev() {
            if ratings[i] > ratings[i + 1] {
                right[i] = right[i + 1] + 1;
            }
        }

        let mut result = 0;
        for i in 0..n {
            result += max(left[i], right[i]);
        }
        return result;
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::candy(vec![1, 0, 2])); // 5
    println!("{}", Solution::candy(vec![1, 2, 2])); // 4
}