use std::collections::HashMap;
use std::cmp::min;

impl Solution {
    pub fn min_days(n: i32) -> i32 {
        Solution::min_days_dp(n, &mut HashMap::new())
    }

    fn min_days_dp(n: i32, map: &mut HashMap<i32, i32>) -> i32 {
        if n <= 1 {
            return n;
        }

        if map.contains_key(&n) {
            return map[&n];
        }

        let res = 1 + min(n % 2 + Solution::min_days_dp(n / 2, map), n % 3 + Solution::min_days_dp(n / 3, map));

        map.insert(n, res);

        res
    }
}

struct Solution {}

fn main() {}