use std::cmp::{min, max};
use std::iter;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let mut left: Vec<i32> = iter::repeat(0).take(len).collect();
        let mut right: Vec<i32> = iter::repeat(0).take(len).collect();

        let mut mininum = prices[0];
        let mut maxinum = prices[len - 1];
        for i in 1..len {
            let it = prices[i];
            mininum = min(mininum, it);
            left[i] = max(left[i - 1], it - mininum);
        }
        for i in (0..(len - 1)).rev() {
            let it = prices[i];
            maxinum = max(maxinum, it);
            right[i] = max(right[i + 1], maxinum - it);
        }

        let mut result = 0;
        for i in 0..len {
            result = max(result, left[i] + right[i]);
        }

        result
    }
}

struct Solution {}

fn main() {}