use std::cmp::{min, max};
use std::iter;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if prices.len() == 0 || k == 0 {
            return 0;
        }

        let len = prices.len();
        let mut old: Vec<i32> = iter::repeat(0).take(len).collect();
        let mut new: Vec<i32> = iter::repeat(0).take(len).collect();

        for _ in 1..(k + 1) {

            // dp[i][j] = max(dp[i][j-1], dp[i-1][k] + price[j] - price[k])
            // max profile for i transactions at j day
            let mut max_val = old[0] - prices[0];

            for j in 1..len {
                max_val = max(max_val, old[j] - prices[j]); // dp[i-1][k] - price[k]
                new[j] = max(new[j - 1], prices[j] + max_val);
            }

            let temp = old;
            old = new;
            new = temp;
        }

        return old[len - 1];
    }
}

struct Solution {}

fn main() {}