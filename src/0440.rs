use std::cmp::{min};

impl Solution {
    pub fn find_kth_number(n: i32, mut k: i32) -> i32 {
        let mut cur = 1;
        k -= 1;
        while k != 0 {
            let step = calcStep(n, cur, cur + 1);
            if k >= step {
                cur += 1;
                k -= step;
            } else {
                cur *= 10;
                k -= 1;
            }
        }

        cur
    }
}

fn calcStep(n: i32, n1: i32, n2: i32) -> i32 {
    let n = n as i64;
    let mut n1 = n1 as i64;
    let mut n2 = n2 as i64;

    let mut step = 0;
    while n1 <= n {
        step += min(n2, n + 1) - n1;
        n1 *= 10;
        n2 *= 10;
    }

    step as i32
}

struct Solution {}

fn main() {}